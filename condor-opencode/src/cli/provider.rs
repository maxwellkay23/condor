use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct ProviderArgs {
    #[command(subcommand)]
    pub command: ProviderCommand,
}

#[derive(Subcommand)]
pub enum ProviderCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Auth {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    OauthAuthorize {
        provider_id: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    OauthCallback {
        provider_id: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &ProviderArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        ProviderCommand::List {
            directory,
            workspace,
        } => {
            let result = client
                .provider
                .list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ProviderCommand::Auth {
            directory,
            workspace,
        } => {
            let result = client
                .provider
                .auth(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ProviderCommand::OauthAuthorize {
            provider_id,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::ProviderOauthAuthorizeBody = serde_json::from_str(body)?;
            let result = client
                .provider
                .oauth_authorize(
                    provider_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        ProviderCommand::OauthCallback {
            provider_id,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::ProviderOauthCallbackBody = serde_json::from_str(body)?;
            let result = client
                .provider
                .oauth_callback(
                    provider_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{}", result);
        }
    }
    Ok(())
}
