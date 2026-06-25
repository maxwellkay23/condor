use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct ExperimentalArgs {
    #[command(subcommand)]
    pub command: ExperimentalCommand,
}

#[derive(Subcommand)]
pub enum ExperimentalCommand {
    ConsoleGet {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ConsoleListOrgs {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ConsoleSwitchOrg {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ResourceList {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SessionList {
        #[arg(long)]
        cursor: Option<f64>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        limit: Option<f64>,
        #[arg(long)]
        search: Option<String>,
        #[arg(long)]
        start: Option<f64>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceList {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceCreate {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceRemove {
        id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceStatus {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceSyncList {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceWarp {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    WorkspaceAdapterList {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &ExperimentalArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        ExperimentalCommand::ConsoleGet {
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .console_get(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::ConsoleListOrgs {
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .console_list_orgs(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::ConsoleSwitchOrg {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::ExperimentalConsoleSwitchOrgBody = serde_json::from_str(body)?;
            let result = client
                .experimental
                .console_switch_org(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        ExperimentalCommand::ResourceList {
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .resource_list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::SessionList {
            cursor,
            directory,
            limit,
            search,
            start,
            workspace,
        } => {
            let result = client
                .experimental
                .session_list(
                    None,
                    *cursor,
                    directory.as_deref(),
                    *limit,
                    None,
                    search.as_deref(),
                    *start,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::WorkspaceList {
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .workspace_list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::WorkspaceCreate {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::ExperimentalWorkspaceCreateBody = serde_json::from_str(body)?;
            let result = client
                .experimental
                .workspace_create(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::WorkspaceRemove {
            id,
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .workspace_remove(id, directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::WorkspaceStatus {
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .workspace_status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ExperimentalCommand::WorkspaceSyncList {
            directory,
            workspace,
        } => {
            client
                .experimental
                .workspace_sync_list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("ok");
        }
        ExperimentalCommand::WorkspaceWarp {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::ExperimentalWorkspaceWarpBody = serde_json::from_str(body)?;
            client
                .experimental
                .workspace_warp(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("ok");
        }
        ExperimentalCommand::WorkspaceAdapterList {
            directory,
            workspace,
        } => {
            let result = client
                .experimental
                .workspace_adapter_list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
