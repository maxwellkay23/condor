use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};
use std::num::NonZeroU64;

#[derive(clap::Args)]
pub struct FindArgs {
    #[command(subcommand)]
    pub command: FindCommand,
}

#[derive(Subcommand)]
pub enum FindCommand {
    Text {
        pattern: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Files {
        query: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        dirs: Option<bool>,
        #[arg(long)]
        limit: Option<NonZeroU64>,
        #[arg(long)]
        type_: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Symbols {
        query: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &FindArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        FindCommand::Text {
            pattern,
            directory,
            workspace,
        } => {
            let result = client
                .find
                .text(directory.as_deref(), pattern, workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        FindCommand::Files {
            query,
            directory,
            dirs,
            limit,
            type_,
            workspace,
        } => {
            let dirs_val = dirs.map(|d| {
                if d {
                    types::FindFilesDirs::True
                } else {
                    types::FindFilesDirs::False
                }
            });
            let type_val = type_.as_deref().map(|t| match t {
                "file" => types::FindFilesType::File,
                "directory" => types::FindFilesType::Directory,
                _ => types::FindFilesType::File,
            });
            let result = client
                .find
                .files(
                    directory.as_deref(),
                    dirs_val,
                    *limit,
                    query,
                    type_val,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        FindCommand::Symbols {
            query,
            directory,
            workspace,
        } => {
            let result = client
                .find
                .symbols(directory.as_deref(), query, workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
