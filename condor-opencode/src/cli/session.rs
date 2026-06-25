use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct SessionArgs {
    #[command(subcommand)]
    pub command: SessionCommand,
}

#[derive(Subcommand)]
pub enum SessionCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<f64>,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        search: Option<String>,
        #[arg(long)]
        start: Option<f64>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Get {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Create {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Delete {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Update {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Messages {
        session_id: String,
        #[arg(long)]
        before: Option<String>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<i64>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Prompt {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Abort {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Fork {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Share {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Unshare {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Summarize {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Init {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Children {
        session_id: String,
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
    Todo {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Diff {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        message_id: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Message {
        session_id: String,
        message_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    DeleteMessage {
        session_id: String,
        message_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Command {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Shell {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Revert {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    Unrevert {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    PromptAsync {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
}

pub async fn handle(
    args: &SessionArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        SessionCommand::List {
            directory,
            limit,
            path,
            search,
            start,
            workspace,
        } => {
            let result = client
                .session
                .list(
                    directory.as_deref(),
                    *limit,
                    path.as_deref(),
                    None,
                    None,
                    search.as_deref(),
                    *start,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Get {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .get(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Create {
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionCreateBody = serde_json::from_str(body)?;
            let result = client
                .session
                .create(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Delete {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .delete(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        SessionCommand::Update {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionUpdateBody = serde_json::from_str(body)?;
            let result = client
                .session
                .update(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Messages {
            session_id,
            before,
            directory,
            limit,
            workspace,
        } => {
            let result = client
                .session
                .messages(
                    session_id,
                    before.as_deref(),
                    directory.as_deref(),
                    *limit,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Prompt {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionPromptBody = serde_json::from_str(body)?;
            let result = client
                .session
                .prompt(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Abort {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .abort(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        SessionCommand::Fork {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionForkBody = serde_json::from_str(body)?;
            let result = client
                .session
                .fork(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Share {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .share(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Unshare {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .unshare(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Summarize {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionSummarizeBody = serde_json::from_str(body)?;
            let result = client
                .session
                .summarize(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{}", result);
        }
        SessionCommand::Init {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionInitBody = serde_json::from_str(body)?;
            let result = client
                .session
                .init(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{}", result);
        }
        SessionCommand::Children {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .children(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Status {
            directory,
            workspace,
        } => {
            let result = client
                .session
                .status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Todo {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .todo(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Diff {
            session_id,
            directory,
            message_id,
            workspace,
        } => {
            let mid: Option<types::SessionDiffMessageId> =
                message_id.as_ref().and_then(|m| m.parse().ok());
            let result = client
                .session
                .diff(
                    session_id,
                    directory.as_deref(),
                    mid.as_ref(),
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Message {
            session_id,
            message_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .message(
                    session_id,
                    message_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::DeleteMessage {
            session_id,
            message_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .delete_message(
                    session_id,
                    message_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                )
                .await?;
            println!("{}", result);
        }
        SessionCommand::Command {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionCommandBody = serde_json::from_str(body)?;
            let result = client
                .session
                .command(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Shell {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionShellBody = serde_json::from_str(body)?;
            let result = client
                .session
                .shell(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Revert {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionRevertBody = serde_json::from_str(body)?;
            let result = client
                .session
                .revert(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::Unrevert {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .session
                .unrevert(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        SessionCommand::PromptAsync {
            session_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::SessionPromptAsyncBody = serde_json::from_str(body)?;
            client
                .session
                .prompt_async(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("ok");
        }
    }
    Ok(())
}
