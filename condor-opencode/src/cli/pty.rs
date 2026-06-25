use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct PtyArgs {
    #[command(subcommand)]
    pub command: PtyCommand,
}

#[derive(Subcommand)]
pub enum PtyCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Create {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Get {
        pty_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Update {
        pty_id: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Remove {
        pty_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Connect {
        pty_id: String,
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        ticket: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ConnectToken {
        pty_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Shells {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &PtyArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        PtyCommand::List {
            directory,
            workspace,
        } => {
            let result = client
                .pty
                .list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        PtyCommand::Create {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::PtyCreateBody = serde_json::from_str(body)?;
            let result = client
                .pty
                .create(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        PtyCommand::Get {
            pty_id,
            directory,
            workspace,
        } => {
            let result = client
                .pty
                .get(pty_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        PtyCommand::Update {
            pty_id,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::PtyUpdateBody = serde_json::from_str(body)?;
            let result = client
                .pty
                .update(
                    pty_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        PtyCommand::Remove {
            pty_id,
            directory,
            workspace,
        } => {
            let result = client
                .pty
                .remove(pty_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        PtyCommand::Connect {
            pty_id,
            cursor,
            directory,
            ticket,
            workspace,
        } => {
            let result = client
                .pty
                .connect(
                    pty_id,
                    cursor.as_deref(),
                    directory.as_deref(),
                    ticket.as_deref(),
                    workspace.as_deref(),
                )
                .await?;
            println!("{}", result);
        }
        PtyCommand::ConnectToken {
            pty_id,
            directory,
            workspace,
        } => {
            let result = client
                .pty
                .connect_token(pty_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        PtyCommand::Shells {
            directory,
            workspace,
        } => {
            let result = client
                .pty
                .shells(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
