use crate::{
    MystikoCliError, WalletCommand, WalletCommands, WalletCreateCommand,
    WalletExportMnemonicPhraseCommand, WalletImportCommand,
};
use mystiko_core::{AccountHandler, DepositHandler, Mystiko, SynchronizerHandler, WalletHandler};
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, DepositSummary,
    QuoteDepositOptions, SendDepositOptions, UpdateAccountOptions,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_wallet_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: WalletCommand,
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
        WalletCommands::Create(args) => {
            execute_wallet_create_command(mystiko, args, pretty_json).await
        }
        WalletCommands::Import(args) => {
            execute_wallet_import_command(mystiko, args, pretty_json).await
        }
        WalletCommands::ExportMnemonic(args) => {
            execute_wallet_export_mnemonic_phrase_command(mystiko, args).await
        }
    }
}

pub async fn execute_wallet_create_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: WalletCreateCommand,
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
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .build();
    print_wallet(mystiko.wallets.create(&options).await?, pretty_json)
}

pub async fn execute_wallet_import_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: WalletImportCommand,
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
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .mnemonic_phrase(args.mnemonic)
        .build();
    print_wallet(mystiko.wallets.create(&options).await?, pretty_json)
}

pub async fn execute_wallet_export_mnemonic_phrase_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: WalletExportMnemonicPhraseCommand,
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
    let mnemonic_phrase = mystiko
        .wallets
        .export_mnemonic_phrase(&args.password)
        .await?;
    println!("{}", mnemonic_phrase);
    Ok(())
}

fn print_wallet(wallet: Wallet, pretty_json: bool) -> Result<(), MystikoCliError> {
    if pretty_json {
        println!("{}", serde_json::to_string_pretty(&wallet)?);
    } else {
        println!("{}", serde_json::to_string(&wallet)?);
    }
    Ok(())
}
