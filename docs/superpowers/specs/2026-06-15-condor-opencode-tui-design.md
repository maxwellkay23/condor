# Condor Opencode TUI — Design Spec

## Overview

`condor-opencode-tui` is a new binary crate providing a streaming REPL for the opencode server. It uses Server-Sent Events (SSE) for real-time text output and fast polling as a fallback for structured parts (tool calls, reasoning blocks).

## Architecture

### Concurrent tasks

Two tokio tasks share session state via a `tokio::sync::mpsc` channel:

1. **Main task** — CLI prompt, message submission, polling `session_messages` @ 100ms, printing structured parts
2. **SSE task** — Opens `event_subscribe()` stream, parses SSE frames, sends `EventMessagePartDelta` text deltas via channel

### Flow

1. Parse CLI args (`--session-id`, `--url`)
2. Create or reuse session via `POST /session`
3. Spawn SSE reader task (`event_subscribe` → `ByteStream` → SSE parser)
4. Loop:
   a. Read user input via rustyline (`">>> "` prompt)
   b. Submit via `prompt_async` (`POST /session/{id}/prompt_async`)
   c. Enter poll loop:
      - `select!` on SSE channel (instant text deltas) OR poll timer (100ms)
      - For SSE text deltas: accumulate per `(message_id, part_id)`, print incrementally
      - For poll ticks: fetch `session_messages`, render new `ReasoningPart`/`ToolPart`
      - Detect completion (message status terminal, SSE idle >2s)
   d. Show `">>> "` prompt again
5. Ctrl+C / Ctrl+D → graceful exit

### SSE ByteStream processing

`client.global().event_subscribe()` returns `ResponseValue<ByteStream>`. The inner `ByteStream` implements `futures::Stream<Item = Result<Bytes, reqwest::Error>>`.

Parse SSE frames from raw bytes:

```
event: message_part_delta
data: {"type":"text.delta","sessionID":"ses_xxx","messageID":"msg_xxx","partID":"part_xxx","field":"text","delta":"Hello "}

```

Buffer across chunks. On `\n\n` boundary, parse `event:` and `data:` lines. Deserialize data JSON. Filter for `EventMessagePartDelta` with matching `sessionID`.

### Part rendering

| Event/Part | Display |
|---|---|
| SSE `EventMessagePartDelta` (text) | Print delta incrementally (no newline until full) |
| `Part::ReasoningPart` (from poll) | Print dimmed text |
| `Part::ToolPart::Completed` | Print `[tool_name] output` |
| `Part::ToolPart::Error` | Print `[tool error] message` |
| `Part::TextPart` (from poll) | Print full text (non-streaming parts) |

### Message completion detection

- SSE idle: no events for current message for 2 seconds → check `session_messages`
- Poll: `session_messages` shows assistant message with `finish_reason` or `error`

### CLI interface

```sh
condor-opencode-tui                              # new session, default URL
condor-opencode-tui --session-id ses_xxxx        # resume existing session
condor-opencode-tui --url http://localhost:4096   # custom server URL
```

### Crate structure

```
condor-opencode-tui/
├── Cargo.toml
└── src/
    └── main.rs
```

Dependencies: `condor-common`, `tokio`, `clap`, `futures.workspace`, `rustyline`.

### Error handling

- Connection / SSE errors: print to stderr, retry SSE connection
- Session create errors: exit with error message
- `CondorOpencodeError` variants used throughout (no `Box<dyn Error>`)