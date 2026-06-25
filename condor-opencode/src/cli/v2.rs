use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct V2Args {
    #[command(subcommand)]
    pub command: V2Command,
}

#[derive(Subcommand)]
pub enum V2Command {
    SessionList {
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<f64>,
        #[arg(long)]
        order: Option<String>,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        search: Option<String>,
        #[arg(long)]
        start: Option<f64>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SessionPrompt {
        session_id: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SessionCompact {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SessionWait {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SessionContext {
        session_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SessionMessages {
        session_id: String,
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<f64>,
        #[arg(long)]
        order: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ModelList,
    ProviderList,
    ProviderGet {
        provider_id: String,
    },
}

pub async fn handle(args: &V2Args, client: &OpencodeClient<'_>) -> Result<(), CondorOpencodeError> {
    match &args.command {
        V2Command::SessionList {
            cursor,
            directory,
            limit,
            order,
            path,
            search,
            start,
            workspace,
        } => {
            let order_val = order.as_deref().map(|o| match o {
                "asc" => types::V2SessionListOrder::Asc,
                "desc" => types::V2SessionListOrder::Desc,
                _ => types::V2SessionListOrder::Asc,
            });
            let result = client
                .v2
                .session_list(
                    cursor.as_deref(),
                    directory.as_deref(),
                    *limit,
                    order_val,
                    path.as_deref(),
                    None,
                    search.as_deref(),
                    *start,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        V2Command::SessionPrompt {
            session_id,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::V2SessionPromptBody = serde_json::from_str(body)?;
            let result = client
                .v2
                .session_prompt(
                    session_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        V2Command::SessionCompact {
            session_id,
            directory,
            workspace,
        } => {
            client
                .v2
                .session_compact(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("ok");
        }
        V2Command::SessionWait {
            session_id,
            directory,
            workspace,
        } => {
            client
                .v2
                .session_wait(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("ok");
        }
        V2Command::SessionContext {
            session_id,
            directory,
            workspace,
        } => {
            let result = client
                .v2
                .session_context(session_id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        V2Command::SessionMessages {
            session_id,
            cursor,
            directory,
            limit,
            order,
            workspace,
        } => {
            let order_val = order.as_deref().map(|o| match o {
                "asc" => types::V2SessionMessagesOrder::Asc,
                "desc" => types::V2SessionMessagesOrder::Desc,
                _ => types::V2SessionMessagesOrder::Asc,
            });
            let result = client
                .v2
                .session_messages(
                    session_id,
                    cursor.as_deref(),
                    directory.as_deref(),
                    *limit,
                    order_val,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        V2Command::ModelList => {
            let result = client.v2.model_list(None).await?;
            println!("{:#?}", result);
        }
        V2Command::ProviderList => {
            let result = client.v2.provider_list(None).await?;
            println!("{:#?}", result);
        }
        V2Command::ProviderGet { provider_id } => {
            let result = client.v2.provider_get(provider_id, None).await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
