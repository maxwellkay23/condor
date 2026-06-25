use clap::Parser;

mod cli;

use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), condor_common::CondorOpencodeError> {
    let args = Cli::parse();
    args.execute().await
}
