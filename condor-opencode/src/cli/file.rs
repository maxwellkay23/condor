use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient};

#[derive(clap::Args)]
pub struct FileArgs {
    #[command(subcommand)]
    pub command: FileCommand,
}

#[derive(Subcommand)]
pub enum FileCommand {
    List {
        path: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Read {
        path: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Status {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &FileArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        FileCommand::List {
            path,
            directory,
            workspace,
        } => {
            let result = client
                .file
                .list(directory.as_deref(), path, workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        FileCommand::Read {
            path,
            directory,
            workspace,
        } => {
            let result = client
                .file
                .read(directory.as_deref(), path, workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        FileCommand::Status {
            directory,
            workspace,
        } => {
            let result = client
                .file
                .status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
