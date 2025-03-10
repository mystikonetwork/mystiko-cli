use crate::{
    print_json, MystikoCliError, ScannerAssetsCommand, ScannerBalanceCommand, ScannerCommand,
    ScannerCommands, ScannerImportCommand, ScannerResetCommand, ScannerScanCommand,
    ScannerSyncCommand,
};
use mystiko_core::{Mystiko, ScannerHandler};
use mystiko_protos::core::scanner::v1::{
    AssetImportOptions, AssetImportResult, AssetsByChain, AssetsOptions, BalanceOptions,
    BalanceResult, ScannerResetOptions, ScannerResetResult, ScannerScanOptions, ScannerScanResult,
    ScannerSyncOptions, ScannerSyncResult,
};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_scanner_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    match args.commands {
        ScannerCommands::Sync(args) => {
            execute_scanner_sync_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Scan(args) => {
            execute_scanner_scan_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Reset(args) => {
            execute_scanner_reset_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Import(args) => {
            execute_scanner_import_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Balance(args) => {
            execute_scanner_balance_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Assets(args) => {
            execute_scanner_assets_command(mystiko, args, compact_json).await
        }
    }
}

pub async fn execute_scanner_sync_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerSyncCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.sync(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_scan_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerScanCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.scan(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_reset_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerResetCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.reset(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_import_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerImportCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.import(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_balance_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerBalanceCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.balance(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_assets_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: ScannerAssetsCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScannerSyncOptions,
        ScannerSyncResult,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    >,
    MystikoCliError: From<R::Error>,
{
    let chain_ids = args.chain_id.clone().unwrap_or_default();
    if chain_ids.is_empty() {
        let result = mystiko.scanner.assets(args.into()).await?;
        print_json(&result, compact_json)
    } else {
        let results = chain_ids
            .into_iter()
            .map(|chain_id| mystiko.scanner.chain_assets(chain_id, args.clone().into()))
            .collect::<Vec<_>>();
        let results = futures::future::try_join_all(results).await?;
        print_json(&results, compact_json)
    }
}
