mod args;
mod error;
mod executor;

pub use args::*;
pub use error::*;
pub use executor::*;

use log::LevelFilter;
use mystiko_core::{
    AccountHandler, Database, DepositHandler, Mystiko, MystikoOptions, SynchronizerHandler,
    WalletHandler,
};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, DepositSummary,
    QuoteDepositOptions, SendDepositOptions, UpdateAccountOptions,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_storage::{SqlStatementFormatter, StatementFormatter, Storage};
use mystiko_storage_sqlite::SqliteStorage;
use std::path::PathBuf;

pub async fn execute(
    args: MystikoCliArgs,
) -> Result<Mystiko<SqlStatementFormatter, SqliteStorage>, MystikoCliError> {
    let _ = env_logger::builder()
        .filter_module("", args.extern_logging_level.parse::<LevelFilter>()?)
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
    execute_with_mystiko(&mystiko, args.commands, args.pretty_json).await?;
    Ok(mystiko)
}

pub async fn execute_with_mystiko<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    commands: MystikoCommands,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    D: DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    >,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<W::Error> + From<A::Error> + From<D::Error> + From<Y::Error>,
{
    match commands {
        MystikoCommands::Wallet(wallet_args) => {
            execute_wallet_command::<F, S, W, A, D, Y>(mystiko, wallet_args, pretty_json).await?
        }
        MystikoCommands::Account(account_args) => {
            execute_account_command::<F, S, W, A, D, Y>(mystiko, account_args, pretty_json).await?
        }
        MystikoCommands::Deposit(deposit_args) => {
            execute_deposit_command::<F, S, W, A, D, Y>(mystiko, deposit_args, pretty_json).await?
        }
        MystikoCommands::Synchronizer(synchronizer_args) => {
            execute_synchronizer::<F, S, W, A, D, Y>(mystiko, synchronizer_args, pretty_json)
                .await?
        }
    }
    Ok(())
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
