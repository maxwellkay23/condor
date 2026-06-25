use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient};

#[derive(clap::Args)]
pub struct LspArgs {
    #[command(subcommand)]
    pub command: LspCommand,
}

#[derive(Subcommand)]
pub enum LspCommand {
    Status {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &LspArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        LspCommand::Status {
            directory,
            workspace,
        } => {
            let result = client
                .lsp
                .status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
