use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/condor_common.proto")?;

    let api_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("openapi.json");

    let src = std::fs::read_to_string(&api_path)?;
    let mut spec: serde_json::Value = serde_json::from_str(&src)?;

    // Preprocess OpenAPI 3.1 -> 3.0
    if let Some(obj) = spec.as_object_mut() {
        if let Some(ver) = obj.get_mut("openapi") {
            if ver.as_str() == Some("3.1.0") {
                *ver = serde_json::json!("3.0.3");
            }
        }
    }
    fix_value(&mut spec);

    // Rename dotted schema names to avoid collisions with camelCase names
    // e.g. Event.tui.toast.show -> Event_tui_toast_show (to not conflict with EventTuiToastShow)
    rename_dotted_schemas(&mut spec);

    // Collapse multiple error responses per operation
    if let Some(obj) = spec.as_object_mut() {
        if let Some(paths) = obj.get_mut("paths") {
            if let Some(paths_map) = paths.as_object_mut() {
                for path_item in paths_map.values_mut() {
                    for op_name in [
                        "get", "put", "post", "delete", "options", "head", "patch", "trace",
                    ] {
                        if let Some(op) = path_item.get_mut(op_name) {
                            collapse_error_responses(op);
                        }
                    }
                }
            }
        }
    }

    // Parse into openapiv3 and generate client
    let openapi: openapiv3::OpenAPI = serde_json::from_value(spec)?;
    let mut generator = progenitor::Generator::default();
    let tokens = generator.generate_tokens(&openapi)?;
    let content = tokens.to_string();

    let out = std::env::var("OUT_DIR")?;
    std::fs::write(Path::new(&out).join("openapi.rs"), &content)?;

    println!("cargo:rerun-if-changed={}", api_path.display());

    Ok(())
}

fn fix_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(excl) = map.get("exclusiveMinimum").and_then(|v| v.as_f64()) {
                map.insert("minimum".into(), serde_json::json!(excl));
                map.insert("exclusiveMinimum".into(), serde_json::json!(true));
            }
            if let Some(excl) = map.get("exclusiveMaximum").and_then(|v| v.as_f64()) {
                map.insert("maximum".into(), serde_json::json!(excl));
                map.insert("exclusiveMaximum".into(), serde_json::json!(true));
            }

            if let Some(types) = map.get("type").and_then(|v| v.as_array()) {
                let non_null: Vec<&serde_json::Value> =
                    types.iter().filter(|t| *t != "null").collect();
                let has_null = types.len() != non_null.len();
                if non_null.len() == 1 {
                    map.insert("type".into(), non_null[0].clone());
                    if has_null {
                        map.insert("nullable".into(), serde_json::json!(true));
                    }
                } else if has_null {
                    map.insert("nullable".into(), serde_json::json!(true));
                }
            }

            if map.get("type").and_then(|v| v.as_str()) == Some("null") {
                map.remove("type");
                map.insert("nullable".into(), serde_json::json!(true));
            }

            if map.get("style").and_then(|v| v.as_str()) == Some("deepObject") {
                map.insert("style".into(), serde_json::json!("form"));
            }

            // Flatten anyOf in response content schemas
            let needs_flatten = map.contains_key("content")
                && map.get("content").and_then(|c| c.as_object()).is_some_and(|mm| {
                    mm.values().any(|m| {
                        m.get("schema")
                            .and_then(|s| s.as_object())
                            .is_some_and(|s| s.contains_key("anyOf"))
                    })
                });

            if needs_flatten {
                if let Some(content) = map.get_mut("content").and_then(|c| c.as_object_mut()) {
                    for media in content.values_mut() {
                        if let Some(schema) = media.get_mut("schema") {
                            if let Some(obj) = schema.as_object() {
                                if obj.contains_key("anyOf") {
                                    let first = obj
                                        .get("anyOf")
                                        .and_then(|a| a.as_array())
                                        .and_then(|a| a.first())
                                        .cloned()
                                        .unwrap_or(serde_json::json!({"type": "object"}));
                                    *schema = first;
                                }
                            }
                        }
                    }
                }
            }

            for v in map.values_mut() {
                fix_value(v);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                fix_value(v);
            }
        }
        _ => {}
    }
}

fn rename_dotted_schemas(spec: &mut serde_json::Value) {
    // Remove all schemas with dots in their names (they collide with camelCase names
    // when converted to Rust type names by typify/progenitor).
    // Also remove their entries from the Event anyOf.
    let dotted: Vec<String> = {
        let schemas_map = spec
            .as_object()
            .and_then(|o| o.get("components"))
            .and_then(|c| c.as_object())
            .and_then(|c| c.get("schemas"))
            .and_then(|s| s.as_object());
        match schemas_map {
            Some(map) => map.keys().filter(|k| k.contains('.')).cloned().collect(),
            None => return,
        }
    };

    for name in &dotted {
        if let Some(obj) = spec.as_object_mut() {
            if let Some(schemas) = obj.get_mut("components").and_then(|c| c.get_mut("schemas")) {
                if let Some(schemas_map) = schemas.as_object_mut() {
                    schemas_map.remove(name.as_str());
                }
            }
        }
        // Remove the ref from any union/discriminator schemas
        remove_dotted_refs(spec, name);
    }
}

fn remove_dotted_refs(spec: &mut serde_json::Value, schema_name: &str) {
    let ref_str = format!("#/components/schemas/{}", schema_name);
    walk_remove_refs(spec, &ref_str);
}

fn walk_remove_refs(value: &mut serde_json::Value, ref_str: &str) {
    match value {
        serde_json::Value::Object(map) => {
            let has_anyof = map.contains_key("anyOf");
            if has_anyof {
                if let Some(arr) = map.get_mut("anyOf").and_then(|a| a.as_array_mut()) {
                    arr.retain(|item| {
                        item.get("$ref").and_then(|r| r.as_str()) != Some(ref_str)
                    });
                }
            }
            for v in map.values_mut() {
                walk_remove_refs(v, ref_str);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                walk_remove_refs(v, ref_str);
            }
        }
        _ => {}
    }
}

fn collapse_error_responses(op: &mut serde_json::Value) {
    if let Some(responses) = op.get_mut("responses") {
        if let Some(status_map) = responses.as_object_mut() {
            let mut first_error_key: Option<String> = None;
            let mut extra_errors: Vec<String> = Vec::new();
            for key in status_map.keys() {
                let k = key.clone();
                if !k.starts_with('2') {
                    if first_error_key.is_none() {
                        first_error_key = Some(k);
                    } else {
                        extra_errors.push(k);
                    }
                }
            }
            for key in &extra_errors {
                status_map.remove(key.as_str());
            }
        }
    }
}
