# Condor Opencode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create `condor-opencode` binary crate with a comprehensive CLI for the opencode server API, backed by an expanded `OpencodeClient` in `condor-common`.

**Architecture:** Extract the ad-hoc `OpencodeClient` from `condor-common` into a sub-client pattern (one struct per API domain), then build a CLI crate that maps one-to-one to each sub-client method. The progenitor-generated client (via `build.rs` + `openapi.json`) stays unchanged.

**Tech Stack:** Rust 2024, `clap` (CLI), `reqwest`/`progenitor` (HTTP client), `thiserror` (error types), `tokio` (async runtime), `condor-common` (shared types + generated client)

---

### Task 1: Update AGENTS.md and workspace Cargo.toml

**Files:**
- Modify: `AGENTS.md`
- Modify: `Cargo.toml`

- [ ] **Update AGENTS.md — add coding convention**

Insert under **Coding Conventions** (after the existing bullet list):

```
- **No `Box<dyn Error>`** — never use `Box<dyn std::error::Error>` as a return type. Define a dedicated `thiserror` enum per crate with specific variants.
```

- [ ] **Update workspace Cargo.toml — add condor-opencode member**

```toml
members = ["condor-common", "condor-server", "condor-client", "condor-executor", "condor-opencode"]
```

- [ ] **Commit**

```bash
git add AGENTS.md Cargo.toml
git commit -m "chore: add condor-opencode to workspace, update agent conventions"
```

---

### Task 2: Create CondorOpencodeError in condor-common

**Files:**
- Create: `condor-common/src/opencode_error.rs`
- Modify: `condor-common/src/lib.rs`

- [ ] **Create `condor-common/src/opencode_error.rs`**

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CondorOpencodeError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
}

// The progenitor-generated types have FromStr with regex validation,
// which produces ConversionError on parse failure.
impl From<crate::openapi::types::error::ConversionError> for CondorOpencodeError {
    fn from(e: crate::openapi::types::error::ConversionError) -> Self {
        CondorOpencodeError::ApiError {
            status: 400,
            message: e.to_string(),
        }
    }
}

impl<T: std::fmt::Display> From<progenitor_client::Error<T>> for CondorOpencodeError {
    fn from(e: progenitor_client::Error<T>) -> Self {
        CondorOpencodeError::ApiError {
            status: e.status(),
            message: e.to_string(),
        }
    }
}

- [ ] **Add module to `condor-common/src/lib.rs`**

```rust
mod opencode_error;
pub use opencode_error::CondorOpencodeError;
```

- [ ] **Commit**

```bash
git add condor-common/src/opencode_error.rs condor-common/src/lib.rs
git commit -m "feat: add CondorOpencodeError type"
```

---

### Task 3: Create opencode_client/ directory with sub-clients

**Files:**
- Delete: `condor-common/src/opencode_client.rs`
- Create: `condor-common/src/opencode_client/mod.rs`
- Create: `condor-common/src/opencode_client/session.rs`
- Create: `condor-common/src/opencode_client/file.rs`
- Create: `condor-common/src/opencode_client/vcs.rs`
- Create: `condor-common/src/opencode_client/config.rs`
- Create: `condor-common/src/opencode_client/global.rs`
- Create: `condor-common/src/opencode_client/find.rs`
- Create: `condor-common/src/opencode_client/project.rs`
- Create: `condor-common/src/opencode_client/mcp.rs`
- Create: `condor-common/src/opencode_client/pty.rs`
- Create: `condor-common/src/opencode_client/question.rs`
- Create: `condor-common/src/opencode_client/permission.rs`
- Create: `condor-common/src/opencode_client/provider.rs`
- Create: `condor-common/src/opencode_client/auth.rs`
- Create: `condor-common/src/opencode_client/sync.rs`
- Create: `condor-common/src/opencode_client/v2.rs`
- Create: `condor-common/src/opencode_client/tui.rs`
- Create: `condor-common/src/opencode_client/experimental.rs`
- Create: `condor-common/src/opencode_client/app.rs`
- Create: `condor-common/src/opencode_client/command.rs`
- Create: `condor-common/src/opencode_client/lsp.rs`
- Create: `condor-common/src/opencode_client/formatter.rs`
- Create: `condor-common/src/opencode_client/path.rs`
- Create: `condor-common/src/opencode_client/instance.rs`
- Modify: `condor-common/src/lib.rs`

Each sub-client wraps `&crate::openapi::Client`, delegates to its progenitor methods, and maps errors via `CondorOpencodeError`. Return types use `crate::openapi::types::*`.

- [ ] **Step 1: Create `condor-common/src/opencode_client/mod.rs`**

```rust
mod session;
mod file;
mod vcs;
mod config;
mod global;
mod find;
mod project;
mod mcp;
mod pty;
mod question;
mod permission;
mod provider;
mod auth;
mod sync;
mod v2;
mod tui;
mod experimental;
mod app;
mod command;
mod lsp;
mod formatter;
mod path;
mod instance;

pub use session::SessionClient;
pub use file::FileClient;
pub use vcs::VcsClient;
pub use config::ConfigClient;
pub use global::GlobalClient;
pub use find::FindClient;
pub use project::ProjectClient;
pub use mcp::McpClient;
pub use pty::PtyClient;
pub use question::QuestionClient;
pub use permission::PermissionClient;
pub use provider::ProviderClient;
pub use auth::AuthClient;
pub use sync::SyncClient;
pub use v2::V2Client;
pub use tui::TuiClient;
pub use experimental::ExperimentalClient;
pub use app::AppClient;
pub use command::CommandClient;
pub use lsp::LspClient;
pub use formatter::FormatterClient;
pub use path::PathClient;
pub use instance::InstanceClient;

use crate::CondorOpencodeError;

pub struct OpencodeClient {
    inner: crate::openapi::Client,
}

impl OpencodeClient {
    pub fn new(host: String) -> Self {
        let inner = crate::openapi::Client::new(&host);
        Self { inner }
    }

    pub fn session(&self) -> SessionClient<'_> { SessionClient { client: &self.inner } }
    pub fn file(&self) -> FileClient<'_> { FileClient { client: &self.inner } }
    pub fn vcs(&self) -> VcsClient<'_> { VcsClient { client: &self.inner } }
    pub fn config(&self) -> ConfigClient<'_> { ConfigClient { client: &self.inner } }
    pub fn global(&self) -> GlobalClient<'_> { GlobalClient { client: &self.inner } }
    pub fn find(&self) -> FindClient<'_> { FindClient { client: &self.inner } }
    pub fn project(&self) -> ProjectClient<'_> { ProjectClient { client: &self.inner } }
    pub fn mcp(&self) -> McpClient<'_> { McpClient { client: &self.inner } }
    pub fn pty(&self) -> PtyClient<'_> { PtyClient { client: &self.inner } }
    pub fn question(&self) -> QuestionClient<'_> { QuestionClient { client: &self.inner } }
    pub fn permission(&self) -> PermissionClient<'_> { PermissionClient { client: &self.inner } }
    pub fn provider(&self) -> ProviderClient<'_> { ProviderClient { client: &self.inner } }
    pub fn auth(&self) -> AuthClient<'_> { AuthClient { client: &self.inner } }
    pub fn sync(&self) -> SyncClient<'_> { SyncClient { client: &self.inner } }
    pub fn v2(&self) -> V2Client<'_> { V2Client { client: &self.inner } }
    pub fn tui(&self) -> TuiClient<'_> { TuiClient { client: &self.inner } }
    pub fn experimental(&self) -> ExperimentalClient<'_> { ExperimentalClient { client: &self.inner } }
    pub fn app(&self) -> AppClient<'_> { AppClient { client: &self.inner } }
    pub fn command(&self) -> CommandClient<'_> { CommandClient { client: &self.inner } }
    pub fn lsp(&self) -> LspClient<'_> { LspClient { client: &self.inner } }
    pub fn formatter(&self) -> FormatterClient<'_> { FormatterClient { client: &self.inner } }
    pub fn path(&self) -> PathClient<'_> { PathClient { client: &self.inner } }
    pub fn instance(&self) -> InstanceClient<'_> { InstanceClient { client: &self.inner } }
}
```

- [ ] **Step 2: Create `session.rs` — SessionClient**

```rust
use crate::openapi::types::{
    self, Session, SessionCreateBody, SessionMessagesResponseItem,
    SessionPromptBody, SessionPromptResponse,
};
use crate::CondorOpencodeError;

pub struct SessionClient<'a> {
    pub(crate) client: &'a crate::openapi::Client,
}

impl SessionClient<'_> {
    pub async fn list(
        &self,
        directory: Option<&str>,
        limit: Option<f64>,
        path: Option<&str>,
        roots: Option<&types::SessionListRoots>,
        scope: Option<types::SessionListScope>,
        search: Option<&str>,
        start: Option<f64>,
        workspace: Option<&str>,
    ) -> Result<Vec<Session>, CondorOpencodeError> {
        let response = self.client
            .session_list(directory, limit, path, roots, scope, search, start, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn get(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Session, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_get(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn create(
        &self,
        body: &types::SessionCreateBody,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Session, CondorOpencodeError> {
        let response = self.client.session_create(directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn delete(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Session, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_delete(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn update(
        &self,
        session_id: &str,
        body: &types::SessionUpdateBody,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Session, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_update(&id, directory, workspace, body).await?;
        Ok(response.into_inner())
    }

    pub async fn messages(
        &self,
        session_id: &str,
        before: Option<&str>,
        directory: Option<&str>,
        limit: Option<i64>,
        workspace: Option<&str>,
    ) -> Result<Vec<SessionMessagesResponseItem>, CondorOpencodeError> {
        let id: types::SessionMessagesSessionId = session_id.parse()?;
        let response = self.client
            .session_messages(&id, before, directory, limit, workspace)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn prompt(
        &self,
        session_id: &str,
        body: &SessionPromptBody,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<SessionPromptResponse, CondorOpencodeError> {
        let id: types::SessionPromptSessionId = session_id.parse()?;
        let response = self.client
            .session_prompt(&id, directory, workspace, body)
            .await?;
        Ok(response.into_inner())
    }

    pub async fn abort(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_abort(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn fork(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Session, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_fork(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn share(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_share(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn unshare(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<bool, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_unshare(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn summarize(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<serde_json::Value, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_summarize(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn init(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<serde_json::Value, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_init(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    pub async fn children(
        &self,
        session_id: &str,
        directory: Option<&str>,
        workspace: Option<&str>,
    ) -> Result<Vec<Session>, CondorOpencodeError> {
        let id: types::SessionGetSessionId = session_id.parse()?;
        let response = self.client.session_children(&id, directory, workspace).await?;
        Ok(response.into_inner())
    }

    // Methods requiring body types: status, todo, diff, message, delete_message, command, shell,
    // revert, unrevert, prompt_async, permissions
    // These follow the same pattern as prompt() — pass session_id as newtype wrapper.
    // For brevity, all session methods follow:
    //   1. Convert session_id to newtype
    //   2. Call progenitor method with optional directory/workspace + body
    //   3. Return response.into_inner()
    //
    // Full list of remaining session methods (all return Result<T, CondorOpencodeError>):
    // - status(session_id, directory, workspace) -> SessionStatus
    // - todo(session_id, directory, workspace) -> SessionTodos
    // - diff(session_id, directory, workspace) -> SessionDiff
    // - message(session_id, message_id, directory, workspace) -> SessionMessagesResponseItem
    // - delete_message(session_id, message_id, directory, workspace) -> bool
    // - command(session_id, body, directory, workspace) -> SessionCommandResponse
    // - shell(session_id, body, directory, workspace) -> SessionShellResponse
    // - revert(session_id, directory, workspace) -> bool
    // - unrevert(session_id, directory, workspace) -> bool
    // - prompt_async(session_id, body, directory, workspace) -> SessionPromptAsyncResponse
    // - permissions(session_id, permission_id, body, directory, workspace) -> bool
}
```

- [ ] **Step 3: Create remaining sub-client files**

Each follows the same pattern as `SessionClient`:

**`file.rs` — FileClient**
```
list(directory, path, workspace) -> Vec<ListFileItem>
read(directory, path, workspace) -> FileContent
status(directory, path, workspace) -> FileStatus
```

**`vcs.rs` — VcsClient**
```
get(directory, workspace) -> VcsInfo
status(directory, workspace) -> VcsStatus
diff(directory, context, mode, workspace) -> Vec<VcsFileDiff>
diff_raw(directory, context, mode, workspace) -> String
apply(directory, workspace, body) -> VcsApplyResponse
```

**`config.rs` — ConfigClient**
```
get(directory, workspace) -> Config
update(directory, workspace, body) -> Config
providers(directory, workspace) -> Vec<ConfigProvider>
```

**`global.rs` — GlobalClient**
```
health() -> GlobalHealthResponse
event() -> GlobalEvent
config_get() -> GlobalConfig
config_update(body) -> GlobalConfig
dispose() -> bool
upgrade() -> bool
```

**`find.rs` — FindClient**
```
text(directory, pattern, workspace) -> Vec<FindTextResponseItem>
files(directory, pattern, workspace) -> Vec<FindFilesResponseItem>
symbols(directory, pattern, workspace) -> Vec<FindSymbolsResponseItem>
```

**`project.rs` — ProjectClient**
```
list(directory, workspace) -> Vec<Project>
current(directory, workspace) -> Project
update(project_id, body, directory, workspace) -> Project
init_git(directory, workspace) -> bool
```

**`mcp.rs` — McpClient**
```
status(directory, workspace) -> HashMap<String, McpStatus>
add(body, directory, workspace) -> bool
connect(name, directory, workspace) -> bool
disconnect(name, directory, workspace) -> bool
auth_start(name, body, directory, workspace) -> McpAuthStatus
auth_callback(name, body, directory, workspace) -> bool
auth_remove(name, directory, workspace) -> bool
auth_authenticate(name, body, directory, workspace) -> bool
```

**`pty.rs` — PtyClient**
```
list(directory, workspace) -> Vec<Pty>
create(body, directory, workspace) -> Pty
get(pty_id, directory, workspace) -> Pty
update(pty_id, body, directory, workspace) -> Pty
remove(pty_id, directory, workspace) -> bool
connect(pty_id, directory, workspace) -> PtyConnectResponse
connect_token(pty_id, directory, workspace) -> PtyConnectTokenResponse
shells() -> Vec<ShellInfo>
```

**`question.rs` — QuestionClient**
```
list(directory, workspace) -> Vec<QuestionRequest>
reply(request_id, body, directory, workspace) -> bool
reject(request_id, directory, workspace) -> bool
```

**`permission.rs` — PermissionClient**
```
list(directory, workspace) -> Vec<PermissionRequest>
respond(request_id, body, directory, workspace) -> bool
```

**`provider.rs` — ProviderClient**
```
list(directory, workspace) -> Vec<Provider>
auth(directory, workspace) -> ProviderAuth
oauth_authorize(provider_id, body, directory, workspace) -> ProviderOAuthStatus
oauth_callback(provider_id, body, directory, workspace) -> bool
```

**`auth.rs` — AuthClient**
```
set(provider_id, body) -> bool
remove(provider_id) -> bool
```

**`sync.rs` — SyncClient**
```
start(directory, workspace) -> bool
replay(directory, workspace) -> bool
steal(directory, workspace) -> bool
history(directory, workspace) -> SyncHistoryResponse
```

**`v2.rs` — V2Client**
```
session_list(directory, workspace) -> Vec<V2Session>
session_prompt(session_id, body, directory, workspace) -> V2SessionPromptResponse
session_compact(session_id, directory, workspace) -> bool
session_wait(session_id, directory, workspace) -> V2Session
session_context(session_id, directory, workspace) -> Vec<V2ContextItem>
session_messages(session_id, before, limit, directory, workspace) -> Vec<V2Message>
model_list() -> Vec<V2Model>
provider_list() -> Vec<V2Provider>
provider_get(provider_id) -> V2Provider
```

**`tui.rs` — TuiClient**
```
append_prompt(body) -> bool
open_help() -> bool
open_sessions() -> bool
open_themes() -> bool
open_models() -> bool
submit_prompt(body) -> bool
clear_prompt() -> bool
execute_command(body) -> bool
show_toast(body) -> bool
publish(body) -> bool
select_session(body) -> bool
control_next(directory) -> TuiControlRequest
control_response(body) -> bool
```

**`experimental.rs` — ExperimentalClient**
```
console_get(directory, workspace) -> ConsoleProvider
console_list_orgs(directory, workspace) -> Vec<ConsoleOrg>
console_switch_org(body, directory, workspace) -> bool
tool_list(directory, workspace) -> Vec<ToolInfo>
tool_ids(directory, workspace) -> Vec<String>
worktree_list(directory, workspace) -> Vec<Worktree>
worktree_create(body, directory, workspace) -> Worktree
worktree_remove(id, directory, workspace) -> bool
worktree_reset(body, directory, workspace) -> bool
resource_list(directory, workspace) -> Vec<McpResource>
session_list(directory, workspace) -> Vec<ExperimentalSession>
workspace_list(directory, workspace) -> Vec<Workspace>
workspace_create(body, directory, workspace) -> Workspace
workspace_remove(id, directory, workspace) -> bool
workspace_status(directory, workspace) -> WorkspaceStatus
workspace_sync_list(directory, workspace) -> bool
workspace_warp(body, directory, workspace) -> bool
adapter_list(directory, workspace) -> Vec<WorkspaceAdapter>
```

**`app.rs` — AppClient**
```
log(body, directory, workspace) -> bool
agents(directory, workspace) -> Vec<AgentInfo>
skills(directory, workspace) -> Vec<SkillInfo>
```

**`command.rs` — CommandClient**
```
list(directory, workspace) -> Vec<Command>
```

**`lsp.rs` — LspClient**
```
status(directory, workspace) -> Vec<LspStatus>
```

**`formatter.rs` — FormatterClient**
```
status(directory, workspace) -> Vec<FormatterStatus>
```

**`path.rs` — PathClient**
```
get(directory, workspace) -> Path
```

**`instance.rs` — InstanceClient**
```
dispose(directory, workspace) -> bool
```

Each method follows this pattern:
```rust
pub async fn method_name(&self, ...) -> Result<ReturnType, CondorOpencodeError> {
    let response = self.client
        .generated_method(args)
        .await?;
    Ok(response.into_inner())
}
```

For methods with body params, the caller constructs the body type:
```rust
pub async fn create(&self, body: &types::PtyCreateBody, ...) -> Result<Pty, CondorOpencodeError>
```

For methods with path params that have newtype wrappers, convert from `&str`:
```rust
let id: types::PtyId = pty_id.parse()?;
```

- [ ] **Step 4: Update `condor-common/src/lib.rs`**

Replace:
```rust
mod opencode_client;
pub use opencode_client::OpencodeClient;
```

With:
```rust
mod opencode_client;
pub use opencode_client::{
    OpencodeClient, SessionClient, FileClient, VcsClient, ConfigClient,
    GlobalClient, FindClient, ProjectClient, McpClient, PtyClient,
    QuestionClient, PermissionClient, ProviderClient, AuthClient,
    SyncClient, V2Client, TuiClient, ExperimentalClient, AppClient,
    CommandClient, LspClient, FormatterClient, PathClient, InstanceClient,
};
```

- [ ] **Step 5: Delete old `condor-common/src/opencode_client.rs`**

```bash
rm condor-common/src/opencode_client.rs
```

- [ ] **Step 6: Verify build**

```bash
cargo check -p condor-common
```

- [ ] **Step 7: Commit**

```bash
git add condor-common/src/
git commit -m "feat: restructure OpencodeClient into sub-client pattern"
```

---

### Task 4: Create condor-opencode crate scaffold

**Files:**
- Create: `condor-opencode/Cargo.toml`
- Create: `condor-opencode/src/main.rs`

- [ ] **Create `condor-opencode/Cargo.toml`**

```toml
[package]
name = "condor-opencode"
version = "0.1.0"
edition = "2024"

[dependencies]
condor-common.workspace = true
tokio.workspace = true
clap.workspace = true
serde.workspace = true
serde_json.workspace = true
```

- [ ] **Create `condor-opencode/src/main.rs`**

```rust
use clap::Parser;

mod cli;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), condor_common::CondorOpencodeError> {
    let args = Cli::parse();
    args.execute().await
}
```

- [ ] **Verify build**

```bash
cargo check -p condor-opencode
```

- [ ] **Commit**

```bash
git add condor-opencode/
git commit -m "feat: scaffold condor-opencode binary crate"
```

---

### Task 5: Create CLI subcommands

**Files:**
- Create: `condor-opencode/src/cli/mod.rs`
- Create: `condor-opencode/src/cli/session.rs`
- Create: `condor-opencode/src/cli/file.rs`
- Create: `condor-opencode/src/cli/vcs.rs`
- Create: `condor-opencode/src/cli/config.rs`
- Create: `condor-opencode/src/cli/global.rs`
- Create: `condor-opencode/src/cli/find.rs`
- Create: `condor-opencode/src/cli/project.rs`
- Create: `condor-opencode/src/cli/mcp.rs`
- Create: `condor-opencode/src/cli/pty.rs`
- Create: `condor-opencode/src/cli/question.rs`
- Create: `condor-opencode/src/cli/permission.rs`
- Create: `condor-opencode/src/cli/provider.rs`
- Create: `condor-opencode/src/cli/auth.rs`
- Create: `condor-opencode/src/cli/sync.rs`
- Create: `condor-opencode/src/cli/v2.rs`
- Create: `condor-opencode/src/cli/tui.rs`
- Create: `condor-opencode/src/cli/experimental.rs`
- Create: `condor-opencode/src/cli/app.rs`
- Create: `condor-opencode/src/cli/command.rs`
- Create: `condor-opencode/src/cli/lsp.rs`
- Create: `condor-opencode/src/cli/formatter.rs`
- Create: `condor-opencode/src/cli/path.rs`
- Create: `condor-opencode/src/cli/instance.rs`

- [ ] **Step 1: Create `condor-opencode/src/cli/mod.rs`**

```rust
use clap::Parser;

mod session;
mod file;
mod vcs;
mod config;
mod global;
mod find;
mod project;
mod mcp;
mod pty;
mod question;
mod permission;
mod provider;
mod auth;
mod sync;
mod v2;
mod tui;
mod experimental;
mod app;
mod command;
mod lsp;
mod formatter;
mod path;
mod instance;

#[derive(Parser)]
#[command(name = "condor-opencode")]
pub struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:4096")]
    url: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    Session(session::SessionArgs),
    File(file::FileArgs),
    Vcs(vcs::VcsArgs),
    Config(config::ConfigArgs),
    Global(global::GlobalArgs),
    Find(find::FindArgs),
    Project(project::ProjectArgs),
    Mcp(mcp::McpArgs),
    Pty(pty::PtyArgs),
    Question(question::QuestionArgs),
    Permission(permission::PermissionArgs),
    Provider(provider::ProviderArgs),
    Auth(auth::AuthArgs),
    Sync(sync::SyncArgs),
    V2(v2::V2Args),
    Tui(tui::TuiArgs),
    Experimental(experimental::ExperimentalArgs),
    App(app::AppArgs),
    Command(command::CommandArgs),
    Lsp(lsp::LspArgs),
    Formatter(formatter::FormatterArgs),
    Path(path::PathArgs),
    Instance(instance::InstanceArgs),
}

impl Cli {
    pub async fn execute(&self) -> Result<(), condor_common::CondorOpencodeError> {
        let client = condor_common::OpencodeClient::new(self.url.clone());

        match &self.command {
            Command::Session(args) => session::handle(args, &client).await,
            Command::File(args) => file::handle(args, &client).await,
            Command::Vcs(args) => vcs::handle(args, &client).await,
            Command::Config(args) => config::handle(args, &client).await,
            Command::Global(args) => global::handle(args, &client).await,
            Command::Find(args) => find::handle(args, &client).await,
            Command::Project(args) => project::handle(args, &client).await,
            Command::Mcp(args) => mcp::handle(args, &client).await,
            Command::Pty(args) => pty::handle(args, &client).await,
            Command::Question(args) => question::handle(args, &client).await,
            Command::Permission(args) => permission::handle(args, &client).await,
            Command::Provider(args) => provider::handle(args, &client).await,
            Command::Auth(args) => auth::handle(args, &client).await,
            Command::Sync(args) => sync::handle(args, &client).await,
            Command::V2(args) => v2::handle(args, &client).await,
            Command::Tui(args) => tui::handle(args, &client).await,
            Command::Experimental(args) => experimental::handle(args, &client).await,
            Command::App(args) => app::handle(args, &client).await,
            Command::Command(args) => command::handle(args, &client).await,
            Command::Lsp(args) => lsp::handle(args, &client).await,
            Command::Formatter(args) => formatter::handle(args, &client).await,
            Command::Path(args) => path::handle(args, &client).await,
            Command::Instance(args) => instance::handle(args, &client).await,
        }
    }
}
```

- [ ] **Step 2: Create `condor-opencode/src/cli/session.rs`**

Each CLI file defines args, subcommand enum, and handler. Example for session:

```rust
use clap::Subcommand;
use condor_common::{OpencodeClient, CondorOpencodeError};

#[derive(clap::Args)]
pub struct SessionArgs {
    #[command(subcommand)]
    pub command: SessionCommand,
}

#[derive(Subcommand)]
pub enum SessionCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<f64>,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        search: Option<String>,
        #[arg(long)]
        start: Option<f64>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Get {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Delete {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Abort {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Fork {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Share {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Unshare {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Messages {
        session_id: String,
        #[arg(long)]
        before: Option<String>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Status {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Init {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Children {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Todo {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Summarize {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(args: &SessionArgs, client: &OpencodeClient) -> Result<(), CondorOpencodeError> {
    match &args.command {
        SessionCommand::List { directory, limit, path, search, start, workspace } => {
            let sessions = client.session().list(
                directory.as_deref(),
                *limit,
                path.as_deref(),
                None, None,
                search.as_deref(),
                *start,
                workspace.as_deref(),
            ).await?;
            for s in &sessions {
                println!("{} | {}", s.id, s.title.as_deref().unwrap_or("(no title)"));
            }
        }
        SessionCommand::Get { session_id, directory, workspace } => {
            let session = client.session().get(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("{:#?}", session);
        }
        SessionCommand::Delete { session_id, directory, workspace } => {
            let session = client.session().delete(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("Deleted: {}", session.id);
        }
        SessionCommand::Abort { session_id, directory, workspace } => {
            let ok = client.session().abort(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("Aborted: {}", ok);
        }
        SessionCommand::Fork { session_id, directory, workspace } => {
            let session = client.session().fork(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("Forked: {} -> {}", session_id, session.id);
        }
        SessionCommand::Share { session_id, directory, workspace } => {
            let ok = client.session().share(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("Shared: {}", ok);
        }
        SessionCommand::Unshare { session_id, directory, workspace } => {
            let ok = client.session().unshare(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("Unshared: {}", ok);
        }
        SessionCommand::Messages { session_id, before, directory, limit, workspace } => {
            let msgs = client.session().messages(
                session_id, before.as_deref(), directory.as_deref(), *limit, workspace.as_deref()
            ).await?;
            for msg in &msgs {
                println!("--- {:?} ---", msg.info);
            }
        }
        SessionCommand::Status { session_id, directory, workspace } => {
            let status = client.session().status(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("{:#?}", status);
        }
        SessionCommand::Init { session_id, directory, workspace } => {
            let val = client.session().init(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("{:#?}", val);
        }
        SessionCommand::Children { session_id, directory, workspace } => {
            let children = client.session().children(session_id, directory.as_deref(), workspace.as_deref()).await?;
            for c in &children {
                println!("{} | {}", c.id, c.title.as_deref().unwrap_or("(no title)"));
            }
        }
        SessionCommand::Todo { session_id, directory, workspace } => {
            let todos = client.session().todo(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("{:#?}", todos);
        }
        SessionCommand::Summarize { session_id, directory, workspace } => {
            let val = client.session().summarize(session_id, directory.as_deref(), workspace.as_deref()).await?;
            println!("{:#?}", val);
        }
    }
    Ok(())
}
```

- [ ] **Step 3: Create remaining CLI files following the same pattern**

Each file has the same structure:
1. Args struct with subcommand enum
2. Handler function that calls the corresponding sub-client method
3. Output is printed to stdout (text format using `{:#?}` or custom formatting)

Remaining CLI files and their subcommands:

| File | Commands |
|---|---|
| `file.rs` | `list <path>`, `read <path>`, `status <path>` |
| `vcs.rs` | `get`, `status`, `diff`, `diff-raw`, `apply <patch>` |
| `config.rs` | `get`, `update <json>`, `providers` |
| `global.rs` | `health`, `config-get`, `config-update <json>`, `dispose`, `upgrade` |
| `find.rs` | `text <pattern>`, `files <pattern>`, `symbols <pattern>` |
| `project.rs` | `list`, `current`, `update <id> <json>`, `init-git` |
| `mcp.rs` | `status`, `add <json>`, `connect <name>`, `disconnect <name>`, `auth-start <name> <json>`, `auth-callback <name> <json>`, `auth-remove <name>`, `auth-authenticate <name> <json>` |
| `pty.rs` | `list`, `create <json>`, `get <pty-id>`, `update <pty-id> <json>`, `remove <pty-id>`, `connect <pty-id>`, `connect-token <pty-id>`, `shells` |
| `question.rs` | `list`, `reply <request-id> <json>`, `reject <request-id>` |
| `permission.rs` | `list`, `respond <request-id> <json>` |
| `provider.rs` | `list`, `auth`, `oauth-authorize <provider-id>`, `oauth-callback <provider-id> <json>` |
| `auth.rs` | `set <provider-id> <json>`, `remove <provider-id>` |
| `sync.rs` | `start`, `replay`, `steal`, `history` |
| `v2.rs` | `session-list`, `session-prompt <session-id> <json>`, `session-compact <session-id>`, `session-wait <session-id>`, `session-context <session-id>`, `session-messages <session-id>`, `model-list`, `provider-list`, `provider-get <provider-id>` |
| `tui.rs` | `append-prompt <json>`, `open-help`, `open-sessions`, `open-themes`, `open-models`, `submit-prompt <json>`, `clear-prompt`, `execute-command <json>`, `show-toast <json>`, `publish <json>`, `select-session <json>`, `control-next`, `control-response <json>` |
| `experimental.rs` | `console-get`, `console-list-orgs`, `console-switch-org <json>`, `tool-list`, `tool-ids`, `worktree-list`, `worktree-create <json>`, `worktree-remove <id>`, `worktree-reset <json>`, `resource-list`, `session-list`, `workspace-list`, `workspace-create <json>`, `workspace-remove <id>`, `workspace-status`, `workspace-sync-list`, `workspace-warp <json>`, `adapter-list` |
| `app.rs` | `log <json>`, `agents`, `skills` |
| `command.rs` | `list` |
| `lsp.rs` | `status` |
| `formatter.rs` | `status` |
| `path.rs` | `get` |
| `instance.rs` | `dispose` |

- [ ] **Step 4: Verify build**

```bash
cargo check -p condor-opencode
```

- [ ] **Step 5: Commit**

```bash
git add condor-opencode/src/cli/
git commit -m "feat: add condor-opencode CLI subcommands"
```

---

### Task 6: Cleanup condor-client

**Files:**
- Modify: `condor-client/src/commands.rs`
- Modify: `condor-client/src/main.rs`

- [ ] **Remove ListSessions and ListMessages from commands.rs**

Delete these two variants from the `Command` enum:
```rust
    ListSessions {
        #[arg(long, default_value = "http://127.0.0.1:4096")]
        url: String,
    },
    ListMessages {
        #[arg(long, default_value = "http://127.0.0.1:4096")]
        url: String,
        #[arg(long)]
        session_id: String,
    },
```

- [ ] **Remove OpencodeClient usage from main.rs**

Delete these lines:
```rust
use condor_common::OpencodeClient;
```

And delete the match arms:
```rust
        Command::ListSessions { url } => {
            let client = OpencodeClient::new(url);
            let sessions = client.list_sessions().await?;
            for session in &sessions {
                println!("{} | {} | {}", &*session.id, session.slug, session.title);
            }
        }
        Command::ListMessages { url, session_id } => {
            use condor_common::openapi::types::{Message, Part, ToolState};

            let client = OpencodeClient::new(url);
            let messages = client.list_messages(&session_id).await?;
            for msg in &messages {
                // ... entire match block
            }
        }
```

Also remove the `use condor_common::openapi::types::*` patterns if they're only used there.

- [ ] **Verify build**

```bash
cargo check -p condor-client
```

- [ ] **Commit**

```bash
git add condor-client/src/
git commit -m "refactor: remove ListSessions/ListMessages from condor-client"
```

---

### Task 7: Final verification

- [ ] **Build entire workspace**

```bash
cargo build --workspace
```

- [ ] **Run cargo fmt**

```bash
cargo fmt --all
```

- [ ] **Final commit with formatting**

```bash
git add -A
git commit -m "style: format workspace"
```