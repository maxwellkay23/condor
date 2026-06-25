# Condor Executor REST API — Design Spec

## Overview

Replace the `opencode run` subprocess call in `condor-executor` with a direct REST API call to the opencode server. Add an `opencode` config field for the server URL. Uses `session.prompt()` (blocking) wrapped in a timeout, with session abort on timeout.

## Config

Add `opencode: String` (required) to the config:

```yaml
server:
  host: "127.0.0.1"
  port: 8080
opencode: "http://127.0.0.1:4096"
timeout: 300
poll_interval: 5
groups:
  - "group-id-1"
```

## Flow

1. Pop job from group, get job + job group (unchanged)
2. Mark job as `Running` (unchanged)
3. Render template with job parameters (unchanged)
4. Connect to opencode server via `OpencodeClient`
5. Create a session via `session.create(None, None, &Default::default())`
6. Print session slug and input text
7. Call `session.prompt()` (blocking) wrapped in `tokio::time::timeout()`
8. On success: print parts (Thought, tool calls, Output), mark job Completed
9. On timeout: call `session.abort()`, mark job Failed
10. On other error: mark job Failed

## Output format

```
session: <slug>
Input: <rendered prompt>
Thought: <reasoning text>
[<tool>] <tool output>
Output: <response text>
job <job_id> completed successfully
```

## Error handling

- `CondorOpencodeError` → convert to string, print to stderr, mark Failed
- Timeout → abort session via `session.abort()`, mark Failed
- Template render errors → unchanged