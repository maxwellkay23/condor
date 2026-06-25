use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient};

#[derive(clap::Args)]
pub struct FormatterArgs {
    #[command(subcommand)]
    pub command: FormatterCommand,
}

#[derive(Subcommand)]
pub enum FormatterCommand {
    Status {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &FormatterArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        FormatterCommand::Status {
            directory,
            workspace,
        } => {
            let result = client
                .formatter
                .status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
