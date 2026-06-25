use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct PermissionArgs {
    #[command(subcommand)]
    pub command: PermissionCommand,
}

#[derive(Subcommand)]
pub enum PermissionCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Respond {
        session_id: String,
        permission_id: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &PermissionArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        PermissionCommand::List {
            directory,
            workspace,
        } => {
            let result = client
                .permission
                .list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        PermissionCommand::Respond {
            session_id,
            permission_id,
            body,
            directory,
            workspace,
        } => {
            let body_val: types::PermissionRespondBody = serde_json::from_str(body)?;
            let result = client
                .permission
                .respond(
                    session_id,
                    permission_id,
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
