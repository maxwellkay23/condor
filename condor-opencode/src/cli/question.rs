use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct QuestionArgs {
    #[command(subcommand)]
    pub command: QuestionCommand,
}

#[derive(Subcommand)]
pub enum QuestionCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Reply {
        request_id: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Reject {
        request_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &QuestionArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        QuestionCommand::List {
            directory,
            workspace,
        } => {
            let result = client
                .question
                .list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        QuestionCommand::Reply {
            request_id,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::QuestionReplyBody = serde_json::from_str(body)?;
            let result = client
                .question
                .reply(
                    request_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{}", result);
        }
        QuestionCommand::Reject {
            request_id,
            directory,
            workspace,
        } => {
            let result = client
                .question
                .reject(request_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
    }
    Ok(())
}
