# Condor Executor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a `condor-executor` crate that continuously polls configured job groups in round-robin order, pops jobs, renders templates, and runs `opencode run` with a configurable timeout, following proper `Created` → `Running` → `Completed`/`Failed` status flow.

**Architecture:** New `condor-executor` crate depending on `condor-common`, `serde_yaml`, `tokio`, and `clap`. The existing `render::render_template` function is moved from `condor-client` to `condor-common` for reuse.

**Tech Stack:** Rust, Tokio (async process + timeout), serde_yaml (config), clap (CLI)

---

### Task 1: Move `render_template` to `condor-common`

**Files:**
- Modify: `condor-common/src/lib.rs` — add `mod render; pub use render::render_template;`
- Create: `condor-common/src/render.rs` — copy from `condor-client/src/render.rs`
- Modify: `condor-client/src/render.rs` — remove module, replace `mod render;` with `use condor_common::render_template;`
- Modify: `condor-common/Cargo.toml` — add `minijinja.workspace = true`

- [ ] **Step 1: Add `minijinja` dependency to `condor-common/Cargo.toml`**

```toml
[package]
name = "condor-common"
version = "0.1.0"
edition = "2024"

[dependencies]
thiserror.workspace = true
serde_yaml.workspace = true
serde.workspace = true
tonic.workspace = true
prost.workspace = true
progenitor.workspace = true
progenitor-client.workspace = true
reqwest.workspace = true
serde_json.workspace = true
regress.workspace = true
futures.workspace = true
minijinja.workspace = true

[build-dependencies]
tonic-build = "0.12"
serde_json.workspace = true
openapiv3 = "2"
progenitor.workspace = true
```

- [ ] **Step 2: Create `condor-common/src/render.rs` with the render function and tests**

```rust
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
```

- [ ] **Step 3: Update `condor-common/src/lib.rs` to export `render_template`**

```rust
pub use error::CondorError;

mod error;

pub mod proto {
    tonic::include_proto!("condor_common");
}

mod grpc_client;
pub use grpc_client::CondorGrpcClient;

mod types;
pub use types::{Job, JobGroup, JobStatus};

mod opencode_client;
pub use opencode_client::OpencodeClient;

mod render;
pub use render::render_template;

pub mod openapi {
    include!(concat!(env!("OUT_DIR"), "/openapi.rs"));
}
```

- [ ] **Step 4: Delete `condor-client/src/render.rs`**

Run: `rm condor-client/src/render.rs`

- [ ] **Step 5: Update `condor-client/src/main.rs` to import from `condor-common`**

Replace:
```rust
mod commands;
mod render;
```

With:
```rust
mod commands;
```

Replace all uses of `render::render_template` with `condor_common::render_template`:

Line 68: change `render::render_template(&group.template, &response.parameters)?` to `condor_common::render_template(&group.template, &response.parameters)?`

Line 101: change `render::render_template(&group.template, &job.parameters)?` to `condor_common::render_template(&group.template, &job.parameters)?`

- [ ] **Step 6: Build and test to verify no regressions**

Run:
```bash
cargo build -p condor-common -p condor-client
cargo test -p condor-common
```
Expected: All builds succeed, all tests pass.

- [ ] **Step 7: Commit**

```bash
git add condor-common/src/render.rs condor-common/src/lib.rs condor-common/Cargo.toml condor-client/src/main.rs
git rm condor-client/src/render.rs
git commit -m "refactor: move render_template to condor-common"
```

---

### Task 2: Add `condor-executor` crate

**Files:**
- Modify: `Cargo.toml` (workspace) — add `condor-executor` to members
- Create: `condor-executor/Cargo.toml`
- Create: `condor-executor/src/main.rs` (stub with just `--config` flag)

- [ ] **Step 1: Add `condor-executor` to workspace members**

In `Cargo.toml`, replace:
```toml
members = ["condor-common", "condor-server", "condor-client"]
```

With:
```toml
members = ["condor-common", "condor-server", "condor-client", "condor-executor"]
```

- [ ] **Step 2: Create `condor-executor/Cargo.toml`**

```toml
[package]
name = "condor-executor"
version = "0.1.0"
edition = "2024"

[dependencies]
condor-common.workspace = true
tokio = { workspace = true, features = ["full"] }
clap = { workspace = true, features = ["derive"] }
serde = { workspace = true, features = ["derive"] }
serde_yaml.workspace = true
```

- [ ] **Step 3: Create `condor-executor/src/main.rs` stub**

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    config: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args = Args::parse();
    println!("condor-executor starting");
    Ok(())
}
```

- [ ] **Step 4: Build to verify**

Run:
```bash
cargo build -p condor-executor
```
Expected: Builds successfully.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml condor-executor/
git commit -m "feat: add condor-executor crate stub"
```

---

### Task 3: Implement config parsing

**Files:**
- Modify: `condor-executor/src/main.rs` — add config struct and parsing

- [ ] **Step 1: Implement config struct and parsing**

Replace the contents of `condor-executor/src/main.rs` with:

```rust
use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    config: String,
}

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    timeout: u64,
    poll_interval: u64,
    groups: Vec<String>,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let contents = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&contents)?;

    println!("condor-executor starting");
    println!("  server: {}:{}", config.server.host, config.server.port);
    println!("  timeout: {}s", config.timeout);
    println!("  poll_interval: {}s", config.poll_interval);
    println!("  groups: {:?}", config.groups);

    Ok(())
}
```

- [ ] **Step 2: Build**

Run:
```bash
cargo build -p condor-executor
```
Expected: Builds successfully.

- [ ] **Step 3: Test with a sample config**

Create a temp config file and run:

```bash
cat > /tmp/executor-test.yaml << 'EOF'
server:
  host: "127.0.0.1"
  port: 8080
timeout: 300
poll_interval: 5
groups:
  - "group-a"
  - "group-b"
EOF
cargo run -p condor-executor -- --config /tmp/executor-test.yaml
```
Expected: Prints config values.

- [ ] **Step 4: Commit**

```bash
git add condor-executor/src/main.rs
git commit -m "feat: implement config parsing for executor"
```

---

### Task 4: Implement the main execution loop

**Files:**
- Modify: `condor-executor/src/main.rs` — add gRPC client, round-robin loop, status flow, process spawning with timeout

- [ ] **Step 1: Implement the full executor**

Replace the contents of `condor-executor/src/main.rs` with:

```rust
use clap::Parser;
use condor_common::CondorGrpcClient;
use condor_common::JobStatus;
use condor_common::render_template;
use serde::Deserialize;
use tokio::time::{sleep, timeout, Duration};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    config: String,
}

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    timeout: u64,
    poll_interval: u64,
    groups: Vec<String>,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let contents = std::fs::read_to_string(&args.config)?;
    let config: Config = serde_yaml::from_str(&contents)?;

    let addr = format!("http://{}:{}", config.server.host, config.server.port);
    let mut client = CondorGrpcClient::connect(&addr).await?;

    println!("condor-executor starting");
    println!("  server: {}", addr);
    println!("  timeout: {}s", config.timeout);
    println!("  poll_interval: {}s", config.poll_interval);
    println!("  groups: {:?}", config.groups);

    loop {
        let mut any_job_processed = false;

        for group_id in &config.groups {
            let pop = client.pop_job_from_group(group_id.clone()).await?;
            if pop.job_id.is_empty() {
                continue;
            }

            any_job_processed = true;
            let job_id = pop.job_id;

            let job = client.get_job(job_id.clone()).await?;
            let group = client.get_job_group(group_id.clone()).await?;

            client
                .update_job_status(job_id.clone(), JobStatus::Running)
                .await?;

            let rendered = match render_template(&group.template, &job.parameters) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("render error for job {}: {}", job_id, e);
                    client
                        .update_job_status(job_id, JobStatus::Failed)
                        .await?;
                    continue;
                }
            };

            let result = run_opencode_with_timeout(&rendered, config.timeout).await;

            match result {
                Ok(exit_status) if exit_status.success() => {
                    client
                        .update_job_status(job_id, JobStatus::Completed)
                        .await?;
                    println!("job {} completed successfully", job_id);
                }
                Ok(exit_status) => {
                    eprintln!("job {} exited with status: {}", job_id, exit_status);
                    client
                        .update_job_status(job_id, JobStatus::Failed)
                        .await?;
                }
                Err(e) => {
                    eprintln!("job {} timed out: {}", job_id, e);
                    client
                        .update_job_status(job_id, JobStatus::Failed)
                        .await?;
                }
            }
        }

        if !any_job_processed {
            sleep(Duration::from_secs(config.poll_interval)).await;
        }
    }
}

async fn run_opencode_with_timeout(
    prompt: &str,
    timeout_secs: u64,
) -> Result<std::process::ExitStatus, tokio::time::error::Elapsed> {
    timeout(
        Duration::from_secs(timeout_secs),
        tokio::process::Command::new("opencode")
            .arg("run")
            .arg(prompt)
            .status(),
    )
    .await
    .map(|r| r?)
}
```

- [ ] **Step 2: Build**

Run:
```bash
cargo build -p condor-executor
```
Expected: Builds successfully.

- [ ] **Step 3: Commit**

```bash
git add condor-executor/src/main.rs
git commit -m "feat: implement executor main loop with timeout and status flow"
```

---

### Task 5: Fix the existing `run-jobs` bug in `condor-client`

The `run-jobs` command currently marks the job as `Completed` before running `opencode`. This should follow the same proper status flow: `Running` → `Completed`/`Failed`.

**Files:**
- Modify: `condor-client/src/main.rs` lines 89-114

- [ ] **Step 1: Fix the status flow in `run-jobs`**

In `condor-client/src/main.rs`, replace lines 89-114 (the `Command::RunJobs` match arm):

```rust
Command::RunJobs { group_id } => {
    let addr = format!("http://{}:{}", args.host, args.port);
    let mut client = CondorGrpcClient::connect(&addr).await?;
    let group = client.get_job_group(group_id.clone()).await?;

    let pop = client.pop_job_from_group(group_id).await?;
    if pop.job_id.is_empty() {
        println!("No jobs in group");
        return Ok(());
    }

    let job = client.get_job(pop.job_id.clone()).await?;
    let rendered = condor_common::render_template(&group.template, &job.parameters)?;

    client
        .update_job_status(pop.job_id.clone(), JobStatus::Running)
        .await?;

    let status = std::process::Command::new("opencode")
        .arg("run")
        .arg(&rendered)
        .status()?;

    if status.success() {
        client
            .update_job_status(pop.job_id, JobStatus::Completed)
            .await?;
        println!("job {} completed successfully", job.id);
    } else {
        eprintln!("opencode run exited with: {}", status);
        client
            .update_job_status(pop.job_id, JobStatus::Failed)
            .await?;
    }
}
```

- [ ] **Step 2: Build**

Run:
```bash
cargo build -p condor-client
```
Expected: Builds successfully.

- [ ] **Step 3: Commit**

```bash
git add condor-client/src/main.rs
git commit -m "fix: run-jobs follows proper status flow before/after execution"
```

---

### Task 6: Run workspace-level build verification

**Files:** None

- [ ] **Step 1: Full workspace build**

Run:
```bash
cargo build --workspace
```
Expected: All crates build successfully.

- [ ] **Step 2: Run all tests**

Run:
```bash
cargo test --workspace
```
Expected: All tests pass.

- [ ] **Step 3: Format check**

Run:
```bash
cargo fmt --all --check
```
Expected: No formatting changes needed.

- [ ] **Step 4: Commit (if any formatting changes needed)**

If `cargo fmt` made changes:
```bash
git add -A
git commit -m "style: cargo fmt"
```
