use crate::{
    print_json, MystikoCliError, SynchronizerCommand, SynchronizerCommands,
    SynchronizerStatusCommand, SynchronizerSyncCommand,
};
use mystiko_core::{Mystiko, SynchronizerHandler};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_synchronizer<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: SynchronizerCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<Y::Error>,
{
    match args.commands {
        SynchronizerCommands::Sync(args) => {
            execute_synchronizer_sync(mystiko, args, compact_json).await
        }
        SynchronizerCommands::Status(args) => {
            execute_synchronizer_status(mystiko, args, compact_json).await
        }
    }
}

pub async fn execute_synchronizer_sync<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: SynchronizerSyncCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<Y::Error>,
{
    mystiko.synchronizer.sync(args.into()).await?;
    let status = mystiko.synchronizer.status(false).await?;
    print_json(&status, compact_json)
}

pub async fn execute_synchronizer_status<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: SynchronizerStatusCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<Y::Error>,
{
    let status = mystiko.synchronizer.status(args.with_contracts).await?;
    print_json(&status, compact_json)
}
