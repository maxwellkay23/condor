use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct AppArgs {
    #[command(subcommand)]
    pub command: AppCommand,
}

#[derive(Subcommand)]
pub enum AppCommand {
    Log {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Agents {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Skills {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &AppArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        AppCommand::Log {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::AppLogBody = serde_json::from_str(body)?;
            let result = client
                .app
                .log(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        AppCommand::Agents {
            directory,
            workspace,
        } => {
            let result = client
                .app
                .agents(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        AppCommand::Skills {
            directory,
            workspace,
        } => {
            let result = client
                .app
                .skills(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
