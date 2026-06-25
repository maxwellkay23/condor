use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct SyncArgs {
    #[command(subcommand)]
    pub command: SyncCommand,
}

#[derive(Subcommand)]
pub enum SyncCommand {
    Start {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Replay {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Steal {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    HistoryList {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &SyncArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        SyncCommand::Start {
            directory,
            workspace,
        } => {
            let result = client
                .sync
                .start(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        SyncCommand::Replay {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::SyncReplayBody = serde_json::from_str(body)?;
            let result = client
                .sync
                .replay(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        SyncCommand::Steal {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::SyncStealBody = serde_json::from_str(body)?;
            let result = client
                .sync
                .steal(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        SyncCommand::HistoryList {
            body,
            directory,
            workspace,
        } => {
            let body_val: std::collections::HashMap<String, u64> = serde_json::from_str(body)?;
            let result = client
                .sync
                .history_list(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
