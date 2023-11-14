use crate::{
    print_json, MystikoCliError, ScannerBalanceCommand, ScannerCommand, ScannerCommands,
    ScannerResetCommand, ScannerScanCommand,
};
use mystiko_core::{Mystiko, ScannerHandler};
use mystiko_protos::core::scanner::v1::{
    BalanceOptions, BalanceResult, ResetOptions, ResetResult, ScanOptions, ScanResult,
};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_scanner_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: ScannerCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScanOptions,
        ScanResult,
        ResetOptions,
        ResetResult,
        BalanceOptions,
        BalanceResult,
    >,
    MystikoCliError: From<R::Error>,
{
    match args.commands {
        ScannerCommands::Scan(args) => {
            execute_scanner_scan_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Reset(args) => {
            execute_scanner_reset_command(mystiko, args, compact_json).await
        }
        ScannerCommands::Balance(args) => {
            execute_scanner_balance_command(mystiko, args, compact_json).await
        }
    }
}

pub async fn execute_scanner_scan_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: ScannerScanCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScanOptions,
        ScanResult,
        ResetOptions,
        ResetResult,
        BalanceOptions,
        BalanceResult,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.scan(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_reset_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: ScannerResetCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScanOptions,
        ScanResult,
        ResetOptions,
        ResetResult,
        BalanceOptions,
        BalanceResult,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.reset(args.into()).await?;
    print_json(&result, compact_json)
}

pub async fn execute_scanner_balance_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: ScannerBalanceCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    R: ScannerHandler<
        ScanOptions,
        ScanResult,
        ResetOptions,
        ResetResult,
        BalanceOptions,
        BalanceResult,
    >,
    MystikoCliError: From<R::Error>,
{
    let result = mystiko.scanner.balance(args.into()).await?;
    print_json(&result, compact_json)
}
