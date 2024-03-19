use crate::{
    print_json, MystikoCliError, SynchronizerCommand, SynchronizerCommands,
    SynchronizerResetCommand, SynchronizerStatusCommand, SynchronizerSyncCommand,
};
use mystiko_core::{Mystiko, SynchronizerHandler};
use mystiko_protos::core::synchronizer::v1::{SynchronizerResetOptions, SyncOptions, SynchronizerStatus};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_synchronizer<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SynchronizerCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus, SynchronizerResetOptions>,
    MystikoCliError: From<Y::Error>,
{
    match args.commands {
        SynchronizerCommands::Sync(args) => {
            execute_synchronizer_sync(mystiko, args, compact_json).await
        }
        SynchronizerCommands::Status(args) => {
            execute_synchronizer_status(mystiko, args, compact_json).await
        }
        SynchronizerCommands::Reset(args) => {
            execute_synchronizer_reset(mystiko, args, compact_json).await
        }
    }
}

pub async fn execute_synchronizer_sync<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SynchronizerSyncCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus, SynchronizerResetOptions>,
    MystikoCliError: From<Y::Error>,
{
    let status = mystiko.synchronizer.sync(args.into()).await?;
    print_json(&status, compact_json)
}

pub async fn execute_synchronizer_status<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SynchronizerStatusCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus, SynchronizerResetOptions>,
    MystikoCliError: From<Y::Error>,
{
    let status = mystiko.synchronizer.status(args.with_contracts).await?;
    print_json(&status, compact_json)
}

pub async fn execute_synchronizer_reset<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SynchronizerResetCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus, SynchronizerResetOptions>,
    MystikoCliError: From<Y::Error>,
{
    let options: SynchronizerResetOptions = args.into();
    let with_contracts = options
        .chains
        .iter()
        .any(|chain| !chain.contract_addresses.is_empty());
    mystiko.synchronizer.reset(options).await?;
    let status = mystiko.synchronizer.status(with_contracts).await?;
    print_json(&status, compact_json)
}
