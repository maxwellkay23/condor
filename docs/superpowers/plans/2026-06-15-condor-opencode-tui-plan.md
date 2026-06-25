# Condor Opencode TUI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create `condor-opencode-tui` binary crate — a streaming REPL that uses SSE for real-time text output and fast polling for structured parts.

**Architecture:** Two concurrent tokio tasks (main REPL loop + SSE reader). SSE events provide instant text deltas via channel. A 100ms poll timer fetches `session_messages` for tool calls, reasoning blocks, and completion detection.

**Tech Stack:** Rust 2024, `condor-common` (OpencodeClient + types), `tokio` (async, channels, select), `clap` (CLI args), `futures` (ByteStream), `rustyline` (readline input)

---

### Task 1: Add workspace member and scaffold crate

**Files:**
- Modify: `Cargo.toml`
- Create: `condor-opencode-tui/Cargo.toml`
- Create: `condor-opencode-tui/src/main.rs`

- [ ] **Update workspace Cargo.toml**

Add `condor-opencode-tui` to members:
```toml
members = ["condor-common", "condor-server", "condor-client", "condor-executor", "condor-opencode", "condor-opencode-tui"]
```

Add `rustyline` to workspace dependencies:
```toml
rustyline = "15"
```

- [ ] **Create `condor-opencode-tui/Cargo.toml`**

```toml
[package]
name = "condor-opencode-tui"
version = "0.1.0"
edition = "2024"

[dependencies]
condor-common.workspace = true
tokio.workspace = true
clap.workspace = true
futures.workspace = true
rustyline.workspace = true
```

- [ ] **Create placeholder `condor-opencode-tui/src/main.rs`**

```rust
fn main() {
    println!("condor-opencode-tui placeholder");
}
```

- [ ] **Verify crate builds**

```bash
cargo check -p condor-opencode-tui
```

- [ ] **Commit**

```bash
git add Cargo.toml condor-opencode-tui/
git commit -m "feat: add condor-opencode-tui workspace member"
```

---

### Task 2: Implement streaming REPL

**Files:**
- Rewrite: `condor-opencode-tui/src/main.rs`

This is the main implementation. All logic lives in one file (~350 lines).

- [ ] **Replace `main.rs` with full implementation**

The program has these sections:

**CLI args:**
```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "http://127.0.0.1:4096")]
    url: String,
    #[arg(long)]
    session_id: Option<String>,
}
```

**Session creation/reuse:**
```rust
async fn get_or_create_session(client: &condor_common::OpencodeClient<'_>, session_id: Option<&str>) -> Result<String, CondorOpencodeError> {
    if let Some(id) = session_id {
        return Ok(id.to_owned());
    }
    let body = types::SessionCreateBody {
        agent: None,
        metadata: Default::default(),
        parent_id: None,
        title: None,
    };
    let session = client.session.create(None, None, &body).await?;
    Ok(session.id.to_string())
}
```

Wait — I need to check `SessionCreateBody` fields first.

Actually, let me check the actual SessionCreateBody fields in the generated code.

Hmm, I know from earlier exploration it has: `agent`, `metadata`, `parent_id`, `title` (from the openapi spec). Let me check the generated struct more carefully.

I already found it earlier:
```
struct SessionCreateBody {
  pub agent: Option<String>,
  pub metadata: serde_json::Map<String, Value>,
  // parent_id, title
}
```

Let me write the plan with the actual types known. I'll need to check the exact fields before writing the code.

Actually, I'm already in build mode. Let me just write the implementation plan and then implement it, adjusting as I go. The plan should be detailed but I'll be flexible during implementation.

Let me write the plan:

**SSE ByteStream reader task:**
```rust
use futures::StreamExt;
use tokio::sync::mpsc;

enum SseEvent {
    TextDelta { session_id: String, message_id: String, part_id: String, delta: String },
}

async fn sse_reader_task(
    mut byte_stream: impl futures::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
    tx: mpsc::UnboundedSender<SseEvent>,
) {
    let mut buffer = Vec::new();
    while let Some(chunk_result) = byte_stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                buffer.extend_from_slice(&chunk);
                // Process complete SSE frames delimited by \n\n
                while let Some(pos) = buffer.windows(2).position(|w| w == b"\n\n") {
                    let frame = buffer[..pos].to_vec();
                    buffer = buffer[pos + 2..].to_vec();
                    if let Some(event) = parse_sse_frame(&frame) {
                        let _ = tx.send(event);
                    }
                }
            }
            Err(_) => break,
        }
    }
}

fn parse_sse_frame(frame: &[u8]) -> Option<SseEvent> {
    let text = std::str::from_utf8(frame).ok()?;
    let mut event_type = "";
    let mut data = "";
    for line in text.lines() {
        if let Some(val) = line.strip_prefix("event: ") {
            event_type = val;
        } else if let Some(val) = line.strip_prefix("data: ") {
            data = val;
        }
    }
    if event_type == "message_part_delta" && !data.is_empty() {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
            let session_id = val["sessionID"].as_str()?.to_owned();
            let message_id = val["messageID"].as_str()?.to_owned();
            let part_id = val["partID"].as_str()?.to_owned();
            let delta = val["delta"].as_str()?.to_owned();
            return Some(SseEvent::TextDelta { session_id, message_id, part_id, delta });
        }
    }
    None
}
```

**Main REPL loop:**

```rust
async fn run_repl(args: Args) -> Result<(), CondorOpenCodeError> {
    let inner_client = condor_common::openapi::Client::new(&args.url);
    let client = condor_common::OpencodeClient::new(&inner_client);

    let session_id = get_or_create_session(&client, args.session_id.as_deref()).await?;
    eprintln!("Session: {}", session_id);

    // Start SSE reader
    let (tx, mut rx) = mpsc::unbounded_channel();
    let stream_response = client.global.event_subscribe(None, None).await?;
    let byte_stream = stream_response.into_inner();
    tokio::spawn(async move {
        sse_reader_task(byte_stream, tx).await;
    });

    // Rustyline setup
    let mut rl = rustyline::DefaultEditor::new()?;
    let _ = rl.load_history(".opencode_tui_history");

    loop {
        let input = match rl.readline(">>> ") {
            Ok(line) => line,
            Err(_) => break, // Ctrl+C/Ctrl+D
        };
        if input.trim().is_empty() { continue; }
        rl.add_history_entry(&input)?;

        // Submit prompt_async
        let part = types::SessionPromptAsyncBodyPartsItem::TextPartInput(types::TextPartInput {
            id: None, ignored: None, metadata: Default::default(), synthetic: None,
            text: input, time: None, type_: types::TextPartInputType::Text,
        });
        let body = types::SessionPromptAsyncBody {
            agent: None, format: None, message_id: None, model: None,
            no_reply: Some(false), parts: vec![part], system: None,
            tools: Default::default(), variant: None,
        };
        client.session.prompt_async(&session_id, None, None, &body).await?;

        // Polling phase
        let poll_duration = tokio::time::Duration::from_millis(100);
        let idle_timeout = tokio::time::Duration::from_secs(2);
        let mut last_event = tokio::time::Instant::now();
        let mut printed_part_ids: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut accumulated_text: std::collections::HashMap<String, String> = std::collections::HashMap::new();

        loop {
            tokio::select! {
                Some(event) = rx.recv() => {
                    match event {
                        SseEvent::TextDelta { session_id: sid, message_id, part_id, delta } => {
                            if sid != session_id { continue; }
                            last_event = tokio::time::Instant::now();
                            let key = format!("{}:{}", message_id, part_id);
                            let text = accumulated_text.entry(key).or_default();
                            text.push_str(&delta);
                            // Print delta to stdout without newline
                            print!("{}", delta);
                            use std::io::Write;
                            std::io::stdout().flush().ok();
                        }
                    }
                }
                _ = tokio::time::sleep(poll_duration) => {
                    // Fetch latest messages
                    let messages = client.session.messages(&session_id, None, None, None, None).await?;
                    for msg in &messages {
                        match &msg.info {
                            types::Message::AssistantMessage(_) => {
                                for part in &msg.parts {
                                    let part_id = get_part_id(part);
                                    if printed_part_ids.contains(&part_id) { continue; }
                                    match part {
                                        types::Part::TextPart(t) => {
                                            // Full text part (not delta streamed)
                                            let key = format!("{}:{}", msg_id(msg), &*t.id);
                                            if !accumulated_text.contains_key(&key) {
                                                println!("\n{}", t.text);
                                            }
                                            printed_part_ids.insert(part_id);
                                        }
                                        types::Part::ReasoningPart(r) => {
                                            // Already printed via SSE deltas; maybe flush remaining
                                            printed_part_ids.insert(part_id);
                                        }
                                        types::Part::ToolPart(t) => match &t.state {
                                            types::ToolState::Completed(c) => {
                                                println!("\n\x1b[33m[{}]\x1b[0m {}", t.tool, c.output);
                                                printed_part_ids.insert(part_id);
                                            }
                                            types::ToolState::Error(e) => {
                                                eprintln!("\n\x1b[31m[{} error]\x1b[0m {}", t.tool, e.error);
                                                printed_part_ids.insert(part_id);
                                            }
                                            _ => {}
                                        }
                                        _ => {}
                                    }
                                }
                                // Check completion
                                if is_message_finished(msg) {
                                    break 'poll;
                                }
                            }
                            _ => {}
                        }
                    }
                    // Idle timeout: no SSE events for 2s
                    if last_event.elapsed() > idle_timeout {
                        // Check if message is done
                        let messages = client.session.messages(&session_id, None, None, None, None).await?;
                        if messages.iter().any(|m| matches!(&m.info, types::Message::AssistantMessage(_))) {
                            break 'poll;
                        }
                    }
                }
            }
        }
    }

    rl.save_history(".opencode_tui_history").ok();
    Ok(())
}
```

**Helper functions:**
```rust
fn get_part_id(part: &types::Part) -> String {
    match part {
        types::Part::TextPart(t) => t.id.to_string(),
        types::Part::ReasoningPart(r) => r.id.to_string(),
        types::Part::ToolPart(t) => t.id.to_string(),
        _ => String::new(),
    }
}

fn msg_id(msg: &types::SessionMessagesResponseItem) -> String {
    match &msg.info {
        types::Message::UserMessage(u) => u.id.to_string(),
        types::Message::AssistantMessage(a) => a.id.to_string(),
    }
}
```

- [ ] **Verify build**

```bash
cargo check -p condor-opencode-tui
```

- [ ] **Fix any compilation errors**, iterating to ensure the build passes.

- [ ] **Commit**

```bash
git add condor-opencode-tui/
git commit -m "feat: implement condor-opencode-tui with SSE streaming"
```

---

### Task 3: Final verification

- [ ] **Full workspace build**

```bash
cargo build --workspace
```

- [ ] **Format**

```bash
cargo fmt --all
```

- [ ] **Commit formatting**

```bash
git add -A
git commit -m "style: format workspace"
```