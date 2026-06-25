use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct McpArgs {
    #[command(subcommand)]
    pub command: McpCommand,
}

#[derive(Subcommand)]
pub enum McpCommand {
    Status {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Add {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Connect {
        name: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Disconnect {
        name: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    AuthStart {
        name: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    AuthCallback {
        name: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    AuthRemove {
        name: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    AuthAuthenticate {
        name: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &McpArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        McpCommand::Status {
            directory,
            workspace,
        } => {
            let result = client
                .mcp
                .status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        McpCommand::Add {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::McpAddBody = serde_json::from_str(body)?;
            let result = client
                .mcp
                .add(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        McpCommand::Connect {
            name,
            directory,
            workspace,
        } => {
            let result = client
                .mcp
                .connect(name, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        McpCommand::Disconnect {
            name,
            directory,
            workspace,
        } => {
            let result = client
                .mcp
                .disconnect(name, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        McpCommand::AuthStart {
            name,
            directory,
            workspace,
        } => {
            let result = client
                .mcp
                .auth_start(name, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        McpCommand::AuthCallback {
            name,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::McpAuthCallbackBody = serde_json::from_str(body)?;
            let result = client
                .mcp
                .auth_callback(name, directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        McpCommand::AuthRemove {
            name,
            directory,
            workspace,
        } => {
            let result = client
                .mcp
                .auth_remove(name, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        McpCommand::AuthAuthenticate {
            name,
            directory,
            workspace,
        } => {
            let result = client
                .mcp
                .auth_authenticate(name, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
