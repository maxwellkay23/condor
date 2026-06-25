use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct GlobalArgs {
    #[command(subcommand)]
    pub command: GlobalCommand,
}

#[derive(Subcommand)]
pub enum GlobalCommand {
    Health,
    ConfigGet,
    ConfigUpdate {
        #[arg(long)]
        body: String,
    },
    Dispose,
    Upgrade {
        #[arg(long)]
        body: String,
    },
}

pub async fn handle(
    args: &GlobalArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        GlobalCommand::Health => {
            let result = client.global.health().await?;
            println!("{:#?}", result);
        }
        GlobalCommand::ConfigGet => {
            let result = client.global.config_get().await?;
            println!("{:#?}", result);
        }
        GlobalCommand::ConfigUpdate { body } => {
            let body_val: types::Config = serde_json::from_str(body)?;
            let result = client.global.config_update(&body_val).await?;
            println!("{:#?}", result);
        }
        GlobalCommand::Dispose => {
            let result = client.global.dispose().await?;
            println!("{}", result);
        }
        GlobalCommand::Upgrade { body } => {
            let body_val: types::GlobalUpgradeBody = serde_json::from_str(body)?;
            let result = client.global.upgrade(&body_val).await?;
            println!("{:#?}", result);
        }
    }
    Ok(())
}
