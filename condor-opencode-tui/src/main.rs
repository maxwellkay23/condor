use clap::Parser;
use condor_common::CondorOpencodeError;
use condor_common::openapi::types;
use futures::StreamExt;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use tokio::sync::mpsc;

#[derive(Parser)]
#[command(name = "condor-opencode-tui")]
struct Args {
    #[arg(long, default_value = "http://127.0.0.1:4096")]
    url: String,

    #[arg(long)]
    session_id: Option<String>,
}

enum SseEvent {
    TextDelta {
        session_id: String,
        #[allow(dead_code)]
        message_id: String,
        part_id: String,
        delta: String,
    },
    SessionDiff {
        session_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), CondorOpencodeError> {
    let args = Args::parse();

    let inner = condor_common::openapi::Client::new(&args.url);
    let client = condor_common::OpencodeClient::new(&inner);

    let session_id = if let Some(id) = &args.session_id {
        id.clone()
    } else {
        let body = types::SessionCreateBody {
            agent: None,
            metadata: Default::default(),
            model: None,
            parent_id: None,
            permission: None,
            title: None,
            workspace_id: None,
        };
        let session = client.session.create(None, None, &body).await?;
        let sid = session.id.to_string();
        eprintln!("Created session: {}", sid);
        sid
    };

    let (tx, rx) = mpsc::unbounded_channel::<SseEvent>();
    let sse_url = args.url.clone();
    let sse_session = session_id.clone();
    tokio::spawn(async move {
        if let Err(e) = run_sse(sse_url, sse_session, tx).await {
            eprintln!("SSE error: {}", e);
        }
    });

    run_repl(&client, &session_id, rx).await
}

async fn run_sse(
    url: String,
    session_id: String,
    tx: mpsc::UnboundedSender<SseEvent>,
) -> Result<(), CondorOpencodeError> {
    let inner = condor_common::openapi::Client::new(&url);
    let response = inner.event_subscribe(None, None).await?;
    let byte_stream = response.into_inner();
    let mut stream = byte_stream.into_inner();

    let mut buffer = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(_) => break,
        };
        buffer.extend_from_slice(&chunk);

        loop {
            let mut found = None;
            for i in 0..buffer.len().saturating_sub(1) {
                if buffer[i] == b'\n' && buffer[i + 1] == b'\n' {
                    found = Some(i);
                    break;
                }
            }
            match found {
                Some(end) => {
                    let frame = buffer[..end].to_vec();
                    buffer = buffer[end + 2..].to_vec();
                    if let Some(event) = parse_sse_frame(&frame) {
                        if event_session_matches(&event, &session_id) {
                            let _ = tx.send(event);
                        }
                    }
                }
                None => break,
            }
        }
    }
    Ok(())
}

fn event_session_matches(event: &SseEvent, session_id: &str) -> bool {
    match event {
        SseEvent::TextDelta {
            session_id: sid, ..
        } => sid == session_id,
        SseEvent::SessionDiff { session_id: sid } => sid == session_id,
    }
}

fn parse_sse_frame(frame: &[u8]) -> Option<SseEvent> {
    let text = std::str::from_utf8(frame).ok()?;
    let mut data = "";
    for line in text.lines() {
        if let Some(val) = line.strip_prefix("data: ") {
            data = val;
            break;
        }
    }
    if data.is_empty() {
        return None;
    }
    let val: serde_json::Value = serde_json::from_str(data).ok()?;
    let event_type = val["type"].as_str()?;
    let props = &val["properties"];
    match event_type {
        "message.part.delta" => {
            let field = props["field"].as_str()?;
            if field != "text" {
                return None;
            }
            Some(SseEvent::TextDelta {
                session_id: props["sessionID"].as_str()?.to_owned(),
                message_id: props["messageID"].as_str()?.to_owned(),
                part_id: props["partID"].as_str()?.to_owned(),
                delta: props["delta"].as_str()?.to_owned(),
            })
        }
        "message.updated" => {
            let info = &val["properties"]["info"];
            if info["finish"].is_string() || info["error"].is_string() {
                Some(SseEvent::SessionDiff {
                    session_id: props["sessionID"].as_str()?.to_owned(),
                })
            } else {
                None
            }
        }
        "session.diff" | "session.status" | "message.part.updated" => Some(SseEvent::SessionDiff {
            session_id: props["sessionID"].as_str()?.to_owned(),
        }),
        _ => None,
    }
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_escape = false;
    for b in s.bytes() {
        if in_escape {
            if b == b'm' || (b >= b'@' && b <= b'~') {
                in_escape = false;
            }
        } else if b == b'\x1b' {
            in_escape = true;
        } else {
            out.push(b as char);
        }
    }
    out
}

fn part_id_str(part: &types::Part) -> String {
    match part {
        types::Part::TextPart(t) => t.id.to_string(),
        types::Part::ReasoningPart(r) => r.id.to_string(),
        types::Part::ToolPart(t) => t.id.to_string(),
        types::Part::FilePart(f) => f.id.to_string(),
        types::Part::SnapshotPart(s) => s.id.to_string(),
        types::Part::PatchPart(p) => p.id.to_string(),
        types::Part::AgentPart(a) => a.id.to_string(),
        types::Part::SubtaskPart(s) => s.id.to_string(),
        types::Part::StepStartPart(s) => s.id.to_string(),
        types::Part::StepFinishPart(s) => s.id.to_string(),
        types::Part::RetryPart(r) => r.id.to_string(),
        types::Part::CompactionPart(c) => c.id.to_string(),
    }
}

fn assistant_msg_id(msg: &types::SessionMessagesResponseItem) -> String {
    match &msg.info {
        types::Message::AssistantMessage(a) => a.id.to_string(),
        types::Message::UserMessage(_) => String::new(),
    }
}

fn is_assistant_finished(msg: &types::SessionMessagesResponseItem) -> bool {
    match &msg.info {
        types::Message::AssistantMessage(a) => a.finish.is_some() || a.error.is_some(),
        _ => false,
    }
}

async fn run_repl(
    client: &condor_common::OpencodeClient<'_>,
    session_id: &str,
    mut rx: mpsc::UnboundedReceiver<SseEvent>,
) -> Result<(), CondorOpencodeError> {
    let mut rl = rustyline::DefaultEditor::new().map_err(|e| CondorOpencodeError::ApiError {
        status: 0,
        message: format!("rustyline init: {}", e),
    })?;
    let _ = rl.load_history(".condor_opencode_history");

    eprintln!("Session: {}", session_id);
    eprintln!("Type your messages. Ctrl+C or Ctrl+D to exit.\n");

    let mut printed: HashSet<String> = HashSet::new();
    let mut accumulator: HashMap<String, String> = HashMap::new();
    let mut last_printed_len: HashMap<String, usize> = HashMap::new();
    let mut thought_header_shown: HashSet<String> = HashSet::new();
    let mut assistant_header_shown: HashSet<String> = HashSet::new();
    let mut seen_assistant_ids: HashSet<String> = HashSet::new();
    let poll_duration = tokio::time::Duration::from_millis(100);

    if let Ok(existing) = client
        .session
        .messages(session_id, None, None, None, None)
        .await
    {
        for msg in &existing {
            for part in &msg.parts {
                let pid = part_id_str(part);
                printed.insert(pid);
            }
        }
    }

    loop {
        let input = match rl.readline(">>> ") {
            Ok(line) => line,
            Err(_) => break,
        };
        if input.trim().is_empty() {
            continue;
        }
        let _ = rl.add_history_entry(&input);

        let part = types::SessionPromptAsyncBodyPartsItem::TextPartInput(types::TextPartInput {
            id: None,
            ignored: None,
            metadata: Default::default(),
            synthetic: None,
            text: input,
            time: None,
            type_: types::TextPartInputType::Text,
        });
        let body = types::SessionPromptAsyncBody {
            agent: None,
            format: None,
            message_id: None,
            model: None,
            no_reply: Some(false),
            parts: vec![part],
            system: None,
            tools: Default::default(),
            variant: None,
        };
        client
            .session
            .prompt_async(session_id, None, None, &body)
            .await?;

        let mut last_text_delta = tokio::time::Instant::now();

        'poll: loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    let now = tokio::time::Instant::now();
                    match event {
                        SseEvent::TextDelta { session_id: _, message_id: _, part_id, delta } => {
                            last_text_delta = now;
                            let text = accumulator.entry(part_id).or_default();
                            text.push_str(&delta);
                        }
                        SseEvent::SessionDiff { .. } => {
                        }
                    }
                }
                _ = tokio::time::sleep(poll_duration) => {
                    let messages = client.session.messages(session_id, None, None, None, None).await?;

                    let mut current_assistant_id: Option<String> = None;
                    for msg in &messages {
                        if !matches!(&msg.info, types::Message::AssistantMessage(_)) {
                            continue;
                        }
                        let mid = assistant_msg_id(msg);
                        if !seen_assistant_ids.contains(&mid) {
                            current_assistant_id = Some(mid);
                            break;
                        }
                    }

                    for msg in &messages {
                        let is_assistant = matches!(&msg.info, types::Message::AssistantMessage(_));
                        if !is_assistant {
                            continue;
                        }
                        let mid = assistant_msg_id(msg);
                        let is_current = Some(&mid) == current_assistant_id.as_ref();

                        for part in &msg.parts {
                            let pid = part_id_str(part);
                            let has_streamed = accumulator.contains_key(&pid);

                            match part {
                                types::Part::TextPart(t) => {
                                    let full = if has_streamed {
                                        accumulator.get(&pid).map(|s| s.as_str()).unwrap_or("")
                                    } else {
                                        t.text.as_str()
                                    };
                                    let prev_len = last_printed_len.get(&pid).copied().unwrap_or(0);
                                    if full.len() > prev_len {
                                        let new_text = &full[prev_len..];
                                        if !assistant_header_shown.contains(&pid) {
                                            assistant_header_shown.insert(pid.clone());
                                            println!("\n\x1b[1mAssistant:\x1b[0m");
                                        }
                                        print!("{}", new_text);
                                        std::io::stdout().flush().ok();
                                        last_printed_len.insert(pid.clone(), full.len());
                                    }
                                    if !has_streamed {
                                        printed.insert(pid);
                                    }
                                }
                                types::Part::ReasoningPart(r) => {
                                    let full = if has_streamed {
                                        accumulator.get(&pid).map(|s| s.as_str()).unwrap_or("")
                                    } else {
                                        r.text.as_str()
                                    };
                                    let prev_len = last_printed_len.get(&pid).copied().unwrap_or(0);
                                    if full.len() > prev_len {
                                        let new_text = &full[prev_len..];
                                        if !thought_header_shown.contains(&pid) {
                                            thought_header_shown.insert(pid.clone());
                                            println!("\n\x1b[1mThought:\x1b[0m");
                                        }
                                        print!("{}", new_text);
                                        std::io::stdout().flush().ok();
                                        last_printed_len.insert(pid.clone(), full.len());
                                    }
                                    if !has_streamed {
                                        printed.insert(pid);
                                    }
                                }
                                types::Part::ToolPart(t) => {
                                    if printed.contains(&pid) {
                                        continue;
                                    }
                                    match &t.state {
                                        types::ToolState::Completed(c) => {
                                            let out = strip_ansi(&c.output);
                                            println!("\n\x1b[33m[{}]\x1b[0m {}", t.tool, out.trim());
                                            printed.insert(pid);
                                        }
                                        types::ToolState::Error(e) => {
                                            let out = strip_ansi(&e.error);
                                            eprintln!("\n\x1b[31m[{} error]\x1b[0m {}", t.tool, out.trim());
                                            printed.insert(pid);
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {
                                    printed.insert(pid);
                                }
                            }
                        }

                        if is_assistant_finished(msg) && is_current {
                            let id = assistant_msg_id(msg);
                            if seen_assistant_ids.insert(id) {
                                println!();
                                break 'poll;
                            }
                        }
                    }

                    if current_assistant_id.is_some()
                        && last_text_delta.elapsed() > tokio::time::Duration::from_secs(5)
                    {
                        let messages = client.session.messages(
                            session_id, None, None, None, None,
                        ).await?;
                        for msg in &messages {
                            if !matches!(&msg.info, types::Message::AssistantMessage(_)) {
                                continue;
                            }
                            let mid = assistant_msg_id(msg);
                            if Some(&mid) == current_assistant_id.as_ref()
                                && is_assistant_finished(msg)
                                && seen_assistant_ids.insert(mid)
                            {
                                println!();
                                break 'poll;
                            }
                        }
                    }
                }
            }
        }
    }

    let _ = rl.save_history(".condor_opencode_history");
    Ok(())
}
