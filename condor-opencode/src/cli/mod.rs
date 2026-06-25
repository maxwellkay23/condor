use clap::Parser;
use condor_common::CondorOpencodeError;

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

pub use app::AppArgs;
pub use auth::AuthArgs;
pub use command::CommandArgs;
pub use config::ConfigArgs;
pub use experimental::ExperimentalArgs;
pub use file::FileArgs;
pub use find::FindArgs;
pub use formatter::FormatterArgs;
pub use global::GlobalArgs;
pub use instance::InstanceArgs;
pub use lsp::LspArgs;
pub use mcp::McpArgs;
pub use path::PathArgs;
pub use permission::PermissionArgs;
pub use project::ProjectArgs;
pub use provider::ProviderArgs;
pub use pty::PtyArgs;
pub use question::QuestionArgs;
pub use session::SessionArgs;
pub use sync::SyncArgs;
pub use tui::TuiArgs;
pub use v2::V2Args;
pub use vcs::VcsArgs;

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
    Session(SessionArgs),
    File(FileArgs),
    Vcs(VcsArgs),
    Config(ConfigArgs),
    Global(GlobalArgs),
    Find(FindArgs),
    Project(ProjectArgs),
    Mcp(McpArgs),
    Pty(PtyArgs),
    Question(QuestionArgs),
    Permission(PermissionArgs),
    Provider(ProviderArgs),
    Auth(AuthArgs),
    Sync(SyncArgs),
    V2(V2Args),
    Tui(TuiArgs),
    Experimental(ExperimentalArgs),
    App(AppArgs),
    Command(CommandArgs),
    Lsp(LspArgs),
    Formatter(FormatterArgs),
    Path(PathArgs),
    Instance(InstanceArgs),
}

impl Cli {
    pub async fn execute(&self) -> Result<(), CondorOpencodeError> {
        let api_client = condor_common::openapi::Client::new(&self.url);
        let client = condor_common::OpencodeClient::new(&api_client);

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
