use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectCommand,
}

#[derive(Subcommand)]
pub enum ProjectCommand {
    List {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Current {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Update {
        project_id: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
        #[arg(long)]
        body: String,
    },
    InitGit {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &ProjectArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        ProjectCommand::List {
            directory,
            workspace,
        } => {
            let result = client
                .project
                .list(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ProjectCommand::Current {
            directory,
            workspace,
        } => {
            let result = client
                .project
                .current(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        ProjectCommand::Update {
            project_id,
            directory,
            workspace,
            body,
        } => {
            let body_val: types::ProjectUpdateBody = serde_json::from_str(body)?;
            let result = client
                .project
                .update(
                    project_id,
                    directory.as_deref(),
                    workspace.as_deref(),
                    &body_val,
                )
                .await?;
            println!("{:#?}", result);
        }
        ProjectCommand::InitGit {
            directory,
            workspace,
        } => {
            let result = client
                .project
                .init_git(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
