use crate::{
    MystikoCliError, SynchronizerCommand, SynchronizerCommands, SynchronizerStatusCommand,
    SynchronizerSyncCommand,
};
use mystiko_core::{AccountHandler, DepositHandler, Mystiko, SynchronizerHandler, WalletHandler};
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, DepositSummary,
    QuoteDepositOptions, SendDepositOptions, UpdateAccountOptions,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_synchronizer<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: SynchronizerCommand,
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
    match args.commands {
        SynchronizerCommands::Sync(args) => {
            execute_synchronizer_sync(mystiko, args, pretty_json).await
        }
        SynchronizerCommands::Status(args) => {
            execute_synchronizer_status(mystiko, args, pretty_json).await
        }
    }
}

pub async fn execute_synchronizer_sync<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: SynchronizerSyncCommand,
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
    mystiko.synchronizer.sync(args.into()).await?;
    print_status(mystiko.synchronizer.status(false).await?, pretty_json)?;
    Ok(())
}

pub async fn execute_synchronizer_status<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: SynchronizerStatusCommand,
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
    print_status(
        mystiko.synchronizer.status(args.with_contracts).await?,
        pretty_json,
    )
}

fn print_status(status: SynchronizerStatus, pretty_json: bool) -> Result<(), MystikoCliError> {
    if pretty_json {
        println!("{}", serde_json::to_string_pretty(&status)?);
    } else {
        println!("{}", serde_json::to_string(&status)?);
    }
    Ok(())
}
