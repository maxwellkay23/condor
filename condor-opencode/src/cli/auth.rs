use clap::Subcommand;
use condor_common::{CondorOpencodeError, OpencodeClient, openapi::types};

#[derive(clap::Args)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub command: AuthCommand,
}

#[derive(Subcommand)]
pub enum AuthCommand {
    Set {
        provider_id: String,
        #[arg(long)]
        body: String,
    },
    Remove {
        provider_id: String,
    },
}

pub async fn handle(
    args: &AuthArgs,
    client: &OpencodeClient<'_>,
) -> Result<(), CondorOpencodeError> {
    match &args.command {
        AuthCommand::Set { provider_id, body } => {
            let body_val: types::Auth = serde_json::from_str(body)?;
            let result = client.auth.set(provider_id, &body_val).await?;
            println!("{}", result);
        }
        AuthCommand::Remove { provider_id } => {
            let result = client.auth.remove(provider_id).await?;
            println!("{}", result);
        }
    }
    Ok(())
}
