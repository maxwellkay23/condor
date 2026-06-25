use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct TuiArgs {
    #[command(subcommand)]
    pub command: TuiCommand,
}

#[derive(Subcommand)]
pub enum TuiCommand {
    AppendPrompt {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    OpenHelp {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    OpenSessions {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    OpenThemes {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    OpenModels {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SubmitPrompt {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ClearPrompt {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ExecuteCommand {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ShowToast {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    Publish {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    SelectSession {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ControlNext {
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
    ControlResponse {
        #[arg(long)]
        body: String,
        #[arg(long)]
        directory: Option<String>,
        #[arg(long)]
        workspace: Option<String>,
    },
}

pub async fn handle(
    args: &TuiArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        TuiCommand::AppendPrompt {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::TuiAppendPromptBody = serde_json::from_str(body)?;
            let result = client
                .tui
                .append_prompt(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        TuiCommand::OpenHelp {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .open_help(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        TuiCommand::OpenSessions {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .open_sessions(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        TuiCommand::OpenThemes {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .open_themes(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        TuiCommand::OpenModels {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .open_models(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        TuiCommand::SubmitPrompt {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .submit_prompt(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        TuiCommand::ClearPrompt {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .clear_prompt(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{}", result);
        }
        TuiCommand::ExecuteCommand {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::TuiExecuteCommandBody = serde_json::from_str(body)?;
            let result = client
                .tui
                .execute_command(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        TuiCommand::ShowToast {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::TuiShowToastBody = serde_json::from_str(body)?;
            let result = client
                .tui
                .show_toast(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        TuiCommand::Publish {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::EventTuiPromptAppend = serde_json::from_str(body)?;
            let result = client
                .tui
                .publish(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        TuiCommand::SelectSession {
            body,
            directory,
            workspace,
        } => {
            let body_val: types::TuiSelectSessionBody = serde_json::from_str(body)?;
            let result = client
                .tui
                .select_session(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
        TuiCommand::ControlNext {
            directory,
            workspace,
        } => {
            let result = client
                .tui
                .control_next(directory.as_deref(), workspace.as_deref())
                .await?;
            println!("{:#?}", result);
        }
        TuiCommand::ControlResponse {
            body,
            directory,
            workspace,
        } => {
            let body_val: serde_json::Value = serde_json::from_str(body)?;
            let result = client
                .tui
                .control_response(directory.as_deref(), workspace.as_deref(), &body_val)
                .await?;
            println!("{}", result);
        }
    }
    Ok(())
}
