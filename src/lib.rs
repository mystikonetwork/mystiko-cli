mod args;
mod error;
mod executor;

pub use args::*;
pub use error::*;
pub use executor::*;

use log::LevelFilter;
use mystiko_core::{Database, Mystiko, MystikoOptions};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::path::PathBuf;

pub async fn execute(
    args: MystikoCliArgs,
) -> Result<Mystiko<SqlStatementFormatter, SqliteStorage>, MystikoCliError> {
    let _ = env_logger::builder()
        .filter_module("", args.extern_logging_level.parse::<LevelFilter>()?)
        .filter_module("mystiko", args.logging_level.parse::<LevelFilter>()?)
        .filter_module("mystiko_core", args.logging_level.parse::<LevelFilter>()?)
        .try_init();
    let database = create_database(args.clone()).await?;
    let config_options = ConfigOptions::builder()
        .is_testnet(args.testnet)
        .is_staging(args.staging)
        .git_revision(args.config_git_revision)
        .file_path(args.config_path)
        .build();
    let options = MystikoOptions::builder()
        .config_options(config_options)
        .build();
    let mystiko = Mystiko::new(database, Some(options)).await?;
    match args.commands {
        MystikoCommands::Wallet(wallet_args) => {
            execute_wallet_command(&mystiko, wallet_args, args.pretty_json).await?
        }
        MystikoCommands::Account(account_args) => {
            execute_account_command(&mystiko, account_args, args.pretty_json).await?
        }
    }
    Ok(mystiko)
}

async fn create_database(
    args: MystikoCliArgs,
) -> Result<Database<SqlStatementFormatter, SqliteStorage>, MystikoCliError> {
    let storage = if args.in_memory {
        SqliteStorage::from_memory().await?
    } else {
        let mode = if args.testnet { "testnet" } else { "mainnet" };
        let default_db_path = dirs::home_dir()
            .unwrap_or(PathBuf::from(""))
            .join(".mystiko")
            .join(mode)
            .join("mystiko.db");
        let db_path = args.db_path.map(PathBuf::from).unwrap_or(default_db_path);
        if let Some(parent) = db_path.parent() {
            if !tokio::fs::try_exists(parent).await? {
                tokio::fs::create_dir_all(parent).await?;
            }
        }
        SqliteStorage::from_path(db_path.to_string_lossy()).await?
    };
    Ok(Database::new(SqlStatementFormatter::sqlite(), storage))
}
