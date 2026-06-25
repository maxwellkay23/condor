use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient};

#[derive(clap::Args)]
pub struct InstanceArgs {
    #[command(subcommand)]
    pub command: InstanceCommand,
}

#[derive(Subcommand)]
pub enum InstanceCommand {
    Dispose {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &InstanceArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        InstanceCommand::Dispose {
            directory,
            workspace,
        } => {
            let result = client
                .instance
                .dispose(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
    }
    Ok(())
}
