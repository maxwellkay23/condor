use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct VcsArgs {
    #[command(subcommand)]
    pub command: VcsCommand,
}

#[derive(Subcommand)]
pub enum VcsCommand {
    Get {
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
    Diff {
        #[arg(long)]
        context: Option<u64>,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long, default_value = "git")]
        mode: String,
        #[arg(long)]
        workspace: Option<String>,
    },
    Apply {
        patch: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &VcsArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        VcsCommand::Get {
            directory,
            workspace,
        } => {
            let result = client
                .vcs
                .get(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        VcsCommand::Status {
            directory,
            workspace,
        } => {
            let result = client
                .vcs
                .status(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        VcsCommand::Diff {
            context,
            directory,
            mode,
            workspace,
        } => {
            let mode_val = match mode.as_str() {
                "git" => types::VcsDiffMode::Git,
                "branch" => types::VcsDiffMode::Branch,
                other => {
                    eprintln!("Unknown diff mode: {other}");
                    return Ok(());
                }
            };
            let result = client
                .vcs
                .diff(
                    *context,
                    directory.as_deref(),
                    mode_val,
                    workspace.as_deref(),
                )
                .await?;
            println!("{:#?}", result);
        }
        VcsCommand::Apply {
            patch,
            directory,
            workspace,
        } => {
            let body = types::VcsApplyBody {
                patch: patch.clone(),
            };
            let result = client
                .vcs
                .apply(directory.as_deref(), workspace.as_deref(), &body)
                .await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
