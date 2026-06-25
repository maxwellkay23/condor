use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommand,
}

#[derive(Subcommand)]
pub enum ConfigCommand {
    Get {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Update {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Providers {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &ConfigArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        ConfigCommand::Get {
            directory,
            workspace,
        } => {
            let result = client
                .config
                .get(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ConfigCommand::Update {
            directory,
            workspace,
            body,
        } => {
            let body_val: types::Config = serde_json::from_str(body)?;
            let result = client
                .config
                .update(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        ConfigCommand::Providers {
            directory,
            workspace,
        } => {
            let result = client
                .config
                .providers(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
