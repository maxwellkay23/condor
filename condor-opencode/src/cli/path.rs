use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient};

#[derive(clap::Args)]
pub struct PathArgs {
    #[command(subcommand)]
    pub command: PathCommand,
}

#[derive(Subcommand)]
pub enum PathCommand {
    Get {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &PathArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        PathCommand::Get {
            directory,
            workspace,
        } => {
            let result = client
                .path
                .get(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
