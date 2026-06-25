use std::collections::HashMap;

pub fn render_template(
    template: &str,
    parameters: &HashMap<String, String>,
) -> Result<String, minijinja::Error> {
    let env = minijinja::Environment::new();
    let tmpl = env.template_from_str(template)?;
    tmpl.render(parameters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_template() {
        let mut params = HashMap::new();
        params.insert("name".to_owned(), "Alice".to_owned());
        params.insert("role".to_owned(), "engineer".to_owned());

        let result = render_template("Hello {{ name }}, you are an {{ role }}.", &params).unwrap();
        assert_eq!(result, "Hello Alice, you are an engineer.");
    }

    #[test]
    fn test_render_with_empty_parameters() {
        let params = HashMap::new();
        let result = render_template("Hello world!", &params).unwrap();
        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_render_with_missing_variable() {
        let params = HashMap::new();
        let result = render_template("Hello {{ name }}!", &params).unwrap();
        assert_eq!(result, "Hello !");
    }

    #[test]
    fn test_render_multiple_parameters() {
        let mut params = HashMap::new();
        params.insert("first".to_owned(), "John".to_owned());
        params.insert("last".to_owned(), "Doe".to_owned());
        params.insert("year".to_owned(), "2024".to_owned());

        let result = render_template("{{ first }} {{ last }} ({{ year }})", &params).unwrap();
        assert_eq!(result, "John Doe (2024)");
    }

    #[test]
    fn test_render_with_if_conditional() {
        let mut params = HashMap::new();
        params.insert("show".to_owned(), "true".to_owned());
        params.insert("message".to_owned(), "Hello".to_owned());

        let result =
            render_template("{% if show == 'true' %}{{ message }}{% endif %}", &params).unwrap();
        assert_eq!(result, "Hello");
    }
}
