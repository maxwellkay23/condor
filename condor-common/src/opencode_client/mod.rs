mod app;
mod auth;
mod command;
mod config;
mod experimental;
mod file;
mod find;
mod formatter;
mod global;
mod instance;
mod lsp;
mod mcp;
mod path;
mod permission;
mod project;
mod provider;
mod pty;
mod question;
mod session;
mod sync;
mod tui;
mod v2;
mod vcs;

pub use app::AppClient;
pub use auth::AuthClient;
pub use command::CommandClient;
pub use config::ConfigClient;
pub use experimental::ExperimentalClient;
pub use file::FileClient;
pub use find::FindClient;
pub use formatter::FormatterClient;
pub use global::GlobalClient;
pub use instance::InstanceClient;
pub use lsp::LspClient;
pub use mcp::McpClient;
pub use path::PathClient;
pub use permission::PermissionClient;
pub use project::ProjectClient;
pub use provider::ProviderClient;
pub use pty::PtyClient;
pub use question::QuestionClient;
pub use session::SessionClient;
pub use sync::SyncClient;
pub use tui::TuiClient;
pub use v2::V2Client;
pub use vcs::VcsClient;

pub struct OpencodeClient<'a> {
    pub session: SessionClient<'a>,
    pub file: FileClient<'a>,
    pub vcs: VcsClient<'a>,
    pub config: ConfigClient<'a>,
    pub global: GlobalClient<'a>,
    pub find: FindClient<'a>,
    pub project: ProjectClient<'a>,
    pub mcp: McpClient<'a>,
    pub pty: PtyClient<'a>,
    pub question: QuestionClient<'a>,
    pub permission: PermissionClient<'a>,
    pub provider: ProviderClient<'a>,
    pub auth: AuthClient<'a>,
    pub sync: SyncClient<'a>,
    pub v2: V2Client<'a>,
    pub tui: TuiClient<'a>,
    pub experimental: ExperimentalClient<'a>,
    pub app: AppClient<'a>,
    pub command: CommandClient<'a>,
    pub lsp: LspClient<'a>,
    pub formatter: FormatterClient<'a>,
    pub path: PathClient<'a>,
    pub instance: InstanceClient<'a>,
}

impl<'a> OpencodeClient<'a> {
    pub fn new(client: &'a crate::openapi::Client) -> Self {
        Self {
            session: SessionClient { client },
            file: FileClient { client },
            vcs: VcsClient { client },
            config: ConfigClient { client },
            global: GlobalClient { client },
            find: FindClient { client },
            project: ProjectClient { client },
            mcp: McpClient { client },
            pty: PtyClient { client },
            question: QuestionClient { client },
            permission: PermissionClient { client },
            provider: ProviderClient { client },
            auth: AuthClient { client },
            sync: SyncClient { client },
            v2: V2Client { client },
            tui: TuiClient { client },
            experimental: ExperimentalClient { client },
            app: AppClient { client },
            command: CommandClient { client },
            lsp: LspClient { client },
            formatter: FormatterClient { client },
            path: PathClient { client },
            instance: InstanceClient { client },
        }
    }

    pub fn inner(&self) -> &'a crate::openapi::Client {
        self.session.client
    }
}
