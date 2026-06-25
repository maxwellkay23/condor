# Condor Executor — Design Spec

## Overview

`condor-executor` is a new workspace crate that continuously polls configured job groups in round-robin order, pops jobs, renders their templates, and executes `opencode run` with a configurable timeout. Jobs follow proper status transitions: `Created` → `Running` → `Completed` or `Failed`.

## Architecture

### New Crate: `condor-executor`

A binary crate depending on:
- `condor-common` — gRPC client, `JobStatus` enum
- `serde_yaml` — config parsing (already in workspace deps)
- `tokio` — async runtime, `tokio::time::timeout`, `tokio::process::Command`
- `clap` — `--config` CLI flag

### Shared Rendering

The `render::render_template` function currently lives in `condor-client/src/render.rs`. It will be moved to `condor-common/src/render.rs` so both `condor-client` and `condor-executor` can use it.

## Configuration

YAML config file specified via `--config` CLI flag:

```yaml
server:
  host: "127.0.0.1"
  port: 8080
timeout: 300
poll_interval: 5
groups:
  - "group-id-1"
  - "group-id-2"
```

| Field | Type | Description |
|---|---|---|
| `server.host` | `String` | gRPC server address |
| `server.port` | `u16` | gRPC server port |
| `timeout` | `u64` | Seconds before killing the `opencode` process |
| `poll_interval` | `u64` | Seconds to sleep between rounds when no jobs were found |
| `groups` | `Vec<String>` | List of job group IDs to poll |

## Main Loop

```text
loop:
  any_job_processed = false
  for each group_id in config.groups:
    pop_job_from_group(group_id)
    if no job: continue
    any_job_processed = true

    get_job(job_id)
    get_job_group(group_id)                  # retrieve template
    update_job_status(job_id, Running)

    rendered = render_template(group.template, job.parameters)
    result = run_with_timeout(opencode run <rendered>, config.timeout)

    if result.ok:
      update_job_status(job_id, Completed)
    else (timeout or non-zero exit):
      update_job_status(job_id, Failed)

  if not any_job_processed:
    sleep config.poll_interval seconds      # backoff before next round
```

With groups [A, B] and A=100 jobs, B=3 jobs, execution order is: A,B,A,B,A,B,A,A,A,A...

## Process Spawning

Uses `tokio::process::Command` with `tokio::time::timeout` wrapping `spawn()` + `wait()`. On timeout, the child process receives `SIGKILL`.

## Status Flow Fix

Current `run-jobs` marks jobs as `Completed` before running `opencode`. The executor follows proper status flow:

1. Pop job → `Created`
2. Before running → `Running`
3. After success → `Completed`
4. After failure/timeout → `Failed`

## Error Handling

- gRPC connection errors: log and retry on next cycle
- Template render errors: mark job `Failed` and continue
- Process spawn errors: mark job `Failed` and continue
- Status update errors: log and continue (best-effort)
