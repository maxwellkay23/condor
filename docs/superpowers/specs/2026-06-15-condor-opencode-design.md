# Condor Opencode — Design Spec

## Overview

Extract the opencode HTTP client from `condor-common` into a dedicated sub-client architecture, build a comprehensive typed wrapper (`OpencodeClient`) covering all 80+ endpoints from the opencode server API, and create a new `condor-opencode` binary crate that exposes every endpoint as CLI subcommands.

Clean up `condor-client` by removing the `ListSessions`/`ListMessages` commands that used the ad-hoc client.

## Scope

- **`condor-common`** — houses `OpencodeClient` with sub-clients per API domain, the progenitor-generated client, and `CondorOpencodeError`. The existing `build.rs` (progenitor generation from `openapi.json`) stays unchanged.
- **`condor-opencode`** — new binary crate, standalone CLI consuming `condor-common`'s client.
- **`condor-client`** — remove `ListSessions` and `ListMessages` subcommands and their `OpencodeClient` imports.

## Architecture

### Sub-Client Pattern

The `OpencodeClient` provides access to sub-clients via methods. Each sub-client is a thin struct that wraps the inner progenitor-generated client:

```
OpencodeClient
  .session()  -> SessionClient  (list, get, create, delete, update, messages, prompt, abort, fork, ...)
  .file()     -> FileClient     (list, read, status)
  .vcs()      -> VcsClient      (get, status, diff, diff_raw, apply)
  .config()   -> ConfigClient   (get, update, providers)
  .global()   -> GlobalClient   (health, event, config_get, config_update, dispose, upgrade)
  .find()     -> FindClient     (text, files, symbols)
  .project()  -> ProjectClient  (list, current, update, init_git)
  .mcp()      -> McpClient      (status, add, connect, disconnect, auth)
  .pty()      -> PtyClient      (list, create, get, update, remove, connect, shells, connect_token)
  .question() -> QuestionClient (list, reply, reject)
  .permission() -> PermissionClient (list, respond)
  .provider() -> ProviderClient (list, auth, oauth_authorize, oauth_callback)
  .auth()     -> AuthClient     (set, remove)
  .sync()     -> SyncClient     (start, replay, steal, history)
  .v2()       -> V2Client       (session_list, session_prompt, session_compact, session_wait, session_context, session_messages, model_list, provider_list, provider_get)
  .tui()      -> TuiClient      (append_prompt, open_help, open_sessions, open_themes, open_models, submit_prompt, clear_prompt, execute_command, show_toast, publish, select_session, control_next, control_response)
  .experimental() -> ExperimentalClient (console, tool, worktree, resource, workspace, adapter)
  .app()      -> AppClient      (log, agents, skills)
  .command()  -> CommandClient  (list)
  .lsp()      -> LspClient      (status)
  .formatter() -> FormatterClient (status)
  .path()     -> PathClient     (get)
  .instance() -> InstanceClient (dispose)
```

### File Layout

```
condor-common/src/
  openapi.rs (generated, already exists)
  opencode_client/
    mod.rs        — OpencodeClient struct + imports
    session.rs    — SessionClient
    file.rs       — FileClient
    vcs.rs        — VcsClient
    config.rs     — ConfigClient
    global.rs     — GlobalClient
    find.rs       — FindClient
    project.rs    — ProjectClient
    mcp.rs        — McpClient
    pty.rs        — PtyClient
    question.rs   — QuestionClient
    permission.rs — PermissionClient
    provider.rs   — ProviderClient
    auth.rs       — AuthClient
    sync.rs       — SyncClient
    v2.rs         — V2Client
    tui.rs        — TuiClient
    experimental.rs — ExperimentalClient
    app.rs        — AppClient
    command.rs    — CommandClient
    lsp.rs        — LspClient
    formatter.rs  — FormatterClient
    path.rs       — PathClient
    instance.rs   — InstanceClient
  opencode_error.rs — CondorOpencodeError
```

### CLI: `condor-opencode`

A binary crate using `clap` with nested subcommands matching the sub-client hierarchy:

```sh
condor-opencode session list
condor-opencode session get <id>
condor-opencode session messages <id>
condor-opencode file read <path>
condor-opencode vcs diff
condor-opencode global health
```

Dotted operation IDs (e.g., `session.messages`, `vcs.diff.raw`) become space-separated nested subcommands. Hyphens used where dots would be confusing (e.g., `diff-raw` instead of `diff.raw`).

Output is printed to stdout as formatted text. Errors printed to stderr, exit code 1.

```
condor-opencode/
  Cargo.toml
  src/
    main.rs
    cli/
      mod.rs
      session.rs
      file.rs
      vcs.rs
      config.rs
      global.rs
      find.rs
      project.rs
      mcp.rs
      pty.rs
      question.rs
      permission.rs
      provider.rs
      auth.rs
      sync.rs
      v2.rs
      tui.rs
      experimental.rs
      app.rs
      command.rs
      lsp.rs
      formatter.rs
      path.rs
      instance.rs
```

Each `cli/*.rs` file defines the clap subcommand enum for that domain and the handler function that calls the corresponding `OpencodeClient` sub-client.

### Error Handling

`CondorOpencodeError` lives in `condor-common/src/opencode_error.rs`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CondorOpencodeError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error ({status}): {message}")]
    ApiError { status: u16, message: String },
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
```

All `OpencodeClient` methods return `Result<T, CondorOpencodeError>`. `Box<dyn std::error::Error>` is never used as a return type.

### AGENTS.md Update

Add under **Coding Conventions**:
> - **No `Box<dyn Error>`** — never use `Box<dyn std::error::Error>` as a return type. Define a dedicated `thiserror` enum per crate with specific variants.

### `condor-client` Cleanup

Remove from `condor-client/src/commands.rs`:
- `ListSessions` variant
- `ListMessages` variant

Remove from `condor-client/src/main.rs`:
- `use condor_common::OpencodeClient`
- The match arms for `ListSessions` and `ListMessages`