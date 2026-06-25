# Condor — Agent Guide

This file documents the project structure, conventions, and patterns for AI coding agents working on Condor.

## Overview

Condor is a Rust workspace with four crates:

| Crate | Description |
|---|---|
| `condor-common` | Shared types, protobuf definitions, gRPC client wrapper, template rendering |
| `condor-server` | gRPC server — handles requests, stores data in Redis |
| `condor-client` | CLI client — connects to the server via gRPC |
| `condor-executor` | Continuous job runner — polls groups, executes jobs with timeout |

## Workspace Setup

- **Workspace root:** `Cargo.toml` at the repo root
- **Edition:** 2024
- **Default dependency versions** are managed via `[workspace.dependencies]` in the root `Cargo.toml`

## Communication Layer (gRPC)

- Protobuf definitions live in `condor-common/proto/condor_common.proto`
- gRPC is implemented via `tonic` + `prost`
- The proto file is compiled at build time by `condor-common/build.rs`
- Generated types are re-exported via `condor_common::proto::*`

### Service: `MessageService`

| RPC | Request → Response | Behavior |
|---|---|---|
| `CreateJobGroup` | `CreateJobGroupRequest { name, template }` → `CreateJobGroupResponse { id }` | Generates a UUID, stores job group in Redis hash `job_group::{id}` with fields `name` and `template` |
| `CreateJob` | `CreateJobRequest { name, group_id, params }` → `CreateJobResponse { id }` | Generates a UUID, stores job in Redis hash `job::{id}` with fields `name`, `group_id`, `status`, and stores params in `job::{id}::params` hash |
| `CreateJobsBatch` | `CreateJobsBatchRequest { group_id, jobs }` → `CreateJobsBatchResponse { ids }` | Creates multiple jobs in a batch, returning all generated IDs |
| `GetJobGroup` | `GetJobGroupRequest { id }` → `GetJobGroupResponse { id, name, template }` | Retrieves a job group by ID |
| `GetJob` | `GetJobRequest { id }` → `GetJobResponse { id, name, group_id, status, params }` | Retrieves a job by ID |
| `ListJobGroups` | `ListJobGroupsRequest {}` → `ListJobGroupsResponse { job_groups }` | Lists all job groups |
| `ListJobs` | `ListJobsRequest { group_id }` → `ListJobsResponse { jobs }` | Lists all jobs in a group |
| `DeleteJobGroup` | `DeleteJobGroupRequest { id }` → `DeleteJobGroupResponse {}` | Deletes a job group and its associated jobs |
| `DeleteJob` | `DeleteJobRequest { id }` → `DeleteJobResponse {}` | Deletes a single job |
| `UpdateJobGroup` | `UpdateJobGroupRequest { id, name, template }` → `UpdateJobGroupResponse {}` | Updates a job group's name and/or template |
| `UpdateJob` | `UpdateJobRequest { id, name, status, params_upsert, params_remove }` → `UpdateJobResponse {}` | Updates a job's metadata (name, status) and/or params |

## CondorGrpcClient

Located in `condor-common/src/grpc_client.rs`. A convenience wrapper around the generated `MessageServiceClient`. Methods are async, take owned `String` arguments, and return the inner response types (unwrapped from `tonic::Response`):

```rust
let mut client = CondorGrpcClient::connect("http://127.0.0.1:8080").await?;
client.create_job_group("name".into(), "template".into()).await?;
```

## Redis Layer

Redis interactions are encapsulated in `condor-server/src/redis_store.rs` behind a `RedisStore` struct:

- **Connection:** Uses `redis::aio::ConnectionManager` (requires `tokio-comp` + `connection-manager` features)
- **Keys:**
  - `{root}::condor::job_group::{id}` — Redis hash with fields `name` and `template`
  - `{root}::condor::jobgroups` — Redis set containing all job group IDs
  - `{root}::condor::job::{id}` — Redis hash with fields `name`, `group_id`, `status`
  - `{root}::condor::job::{id}::params` — Redis hash with user-defined key/value pairs
  - `{root}::condor::job_group::{group_id}::jobs` — Redis set containing job IDs for a group
  - `{root}::condor::polling_groups` — Redis set containing group IDs being polled
  - `{root}::condor::group::{group_id}::polled_jobs` — Redis set containing job IDs that have been dispatched by the executor

### Redis Usage
Use `redis::pipe()` for multi-command operations that should be sent atomically in a single round trip (e.g., setting a hash and adding to a set together). For single commands, call the method directly on the connection (e.g., `conn.rpush::<&str, &str, ()>("messages", message).await?`). Import `redis::AsyncCommands` to bring these methods into scope.

## CLI (condor-client)

Uses `clap` with subcommands:

```sh
cargo run -p condor-client -- create-job-group --name "my-group" --template "{{ name }}"
```

## Server (condor-server)

Accepts `--host`, `--port`, and `--redis-url` flags. Defaults to `0.0.0.0:8080` with `redis://127.0.0.1/`.

```sh
cargo run -p condor-server -- --port 8080
```

## Executor (condor-executor)

Continuously polls configured job groups in round-robin order, pops jobs, renders templates via `condor_common::render_template()`, and executes `opencode run <rendered_prompt>` with a configurable timeout. Jobs follow proper status transitions: `Created` → `Running` → `Completed` or `Failed`. Accepts `--config` flag pointing to a YAML file. Example config:

```yaml
server:
  host: "127.0.0.1"
  port: 8080
timeout: 300           # seconds before killing the opencode process
poll_interval: 5      # seconds to sleep between rounds when no jobs are found
groups:
  - "group-id-1"
  - "group-id-2"
```

Process spawning uses `tokio::process::Command` wrapped by `tokio::time::timeout()` for deadline enforcement.

```sh
cargo run -p condor-executor -- --config /path/to/config.yaml
```

## Adding a New RPC

1. Add the RPC and messages to `condor-common/proto/condor_common.proto`
2. Implement the handler in `condor-server/src/main.rs` (on `MessageServiceImpl`)
3. Add a method to `CondorGrpcClient` in `condor-common/src/grpc_client.rs`
4. Add a subcommand to `condor-client/src/main.rs`
5. If Redis persistence is needed, add a method to `RedisStore` in `condor-server/src/redis_store.rs`

## Coding Conventions

- **Edition 2024 Rust** — use `unsafe` blocks, `impl Trait` in return position, etc. as idiomatic for this edition
- **`cargo fmt`** — always format before committing
- **Error handling** — use `Box<dyn std::error::Error>` in `main`, `tonic::Status` in gRPC handlers, `CondorServerError` in the store layer. Do not use a generic catch-all variant like `Generic(String)`. Instead, add a specific variant for each error category (e.g. `InvalidJobStatus`).
- **Async** — all gRPC methods and Redis operations are async
- **`println!`** for CLI output; async `tokio::io::stdout().write()` for server output
- **No `Box<dyn Error>`** — never use `Box<dyn std::error::Error>` as a return type. Define a dedicated `thiserror` enum per crate with specific variants.
