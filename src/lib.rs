mod about;
mod args;
mod error;
mod executor;

pub use about::*;
pub use args::*;
pub use error::*;
pub use executor::*;

use log::LevelFilter;
use mystiko_core::{
    AccountHandler, Database, DepositHandler, Mystiko, MystikoOptions, ScannerHandler,
    SpendHandler, SynchronizerHandler, WalletHandler,
};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::document::v1::{Account, Deposit, Spend, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateSpendOptions, CreateWalletOptions,
    DepositQuote, DepositSummary, QuoteDepositOptions, QuoteSpendOptions, SendDepositOptions,
    SendSpendOptions, SpendQuote, SpendSummary, UpdateAccountOptions,
};
use mystiko_protos::core::scanner::v1::{
    AssetsByChain, AssetsOptions, BalanceOptions, BalanceResult, ResetResult, ScanOptions,
    ScanResult, ScannerResetOptions,
};
use mystiko_protos::core::synchronizer::v1::{
    SyncOptions, SynchronizerResetOptions, SynchronizerStatus,
};
use mystiko_static_cache::{FileStaticCache, StaticCache};
use mystiko_storage::{SqlStatementFormatter, StatementFormatter, Storage};
use mystiko_storage_sqlite::SqliteStorage;
use serde::Serialize;
use std::path::PathBuf;

pub async fn execute(args: MystikoCliArgs) -> Result<(), MystikoCliError> {
    if let MystikoCommands::About = args.commands {
        print_json(&AboutInfo::default(), false)
    } else {
        let _ = env_logger::builder()
            .filter_module("", args.extern_logging_level.parse::<LevelFilter>()?)
            .filter_module("mystiko_core", args.logging_level.parse::<LevelFilter>()?)
            .try_init();
        let mystiko = create_mystiko(&args).await?;
        execute_with_mystiko(&mystiko, args.commands, args.compact_json).await
    }
}

pub async fn execute_with_mystiko<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    commands: MystikoCommands,
    compact_json: bool,
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
    X: SpendHandler<
        Spend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
    >,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus, SynchronizerResetOptions>,
    R: ScannerHandler<
        ScanOptions,
        ScanResult,
        ScannerResetOptions,
        ResetResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<W::Error>
        + From<A::Error>
        + From<D::Error>
        + From<X::Error>
        + From<Y::Error>
        + From<R::Error>,
{
    match commands {
        MystikoCommands::Wallet(wallet_args) => {
            execute_wallet_command::<F, S, W, A, D, X, Y, R>(mystiko, wallet_args, compact_json)
                .await
        }
        MystikoCommands::Account(account_args) => {
            execute_account_command::<F, S, W, A, D, X, Y, R>(mystiko, account_args, compact_json)
                .await
        }
        MystikoCommands::Deposit(deposit_args) => {
            execute_deposit_command::<F, S, W, A, D, X, Y, R>(mystiko, deposit_args, compact_json)
                .await
        }
        MystikoCommands::Spend(spend_args) => {
            execute_spend_command::<F, S, W, A, D, X, Y, R>(mystiko, spend_args, compact_json).await
        }
        MystikoCommands::Scanner(scanner_args) => {
            execute_scanner_command::<F, S, W, A, D, X, Y, R>(mystiko, scanner_args, compact_json)
                .await
        }
        MystikoCommands::Synchronizer(synchronizer_args) => {
            execute_synchronizer::<F, S, W, A, D, X, Y, R>(mystiko, synchronizer_args, compact_json)
                .await
        }
        _ => Ok(()),
    }
}

pub fn print_json<T: Serialize>(value: &T, compact: bool) -> Result<(), MystikoCliError> {
    if compact {
        println!("{}", serde_json::to_string(value)?);
    } else {
        println!("{}", serde_json::to_string_pretty(value)?);
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

fn static_cache_path(args: &MystikoCliArgs) -> PathBuf {
    let default_db_path = dirs::home_dir()
        .unwrap_or(PathBuf::from(""))
        .join(".mystiko")
        .join("cache");
    args.static_cache_path
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or(default_db_path)
}

async fn create_mystiko(
    args: &MystikoCliArgs,
) -> Result<Mystiko<SqlStatementFormatter, SqliteStorage>, MystikoCliError> {
    let database = create_database(args.clone()).await?;
    let static_cache = FileStaticCache::new(static_cache_path(args)).await?;
    let config_options = ConfigOptions::builder()
        .is_testnet(args.testnet)
        .is_staging(args.staging)
        .git_revision(args.config_git_revision.clone())
        .file_path(args.config_path.clone())
        .build();
    let options = MystikoOptions::builder()
        .config_options(config_options)
        .static_cache(Box::new(static_cache) as Box<dyn StaticCache>)
        .build();
    Ok(Mystiko::new(database, Some(options)).await?)
}
