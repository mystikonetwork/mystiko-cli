use clap::Parser;
use mystiko::{execute, MystikoCliArgs, MystikoCliError};

#[tokio::main]
async fn main() -> Result<(), MystikoCliError> {
    let args = MystikoCliArgs::parse();
    execute(args).await
}
