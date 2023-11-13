use crate::{
    AccountCommand, AccountCommands, AccountCreateCommand, AccountExportSecretKeyCommand,
    AccountImportCommand, MystikoCliError,
};
use mystiko_core::{AccountHandler, DepositHandler, Mystiko, SynchronizerHandler, WalletHandler};
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, DepositSummary,
    QuoteDepositOptions, SendDepositOptions, UpdateAccountOptions,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_account_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: AccountCommand,
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
        AccountCommands::Create(args) => {
            execute_account_create_command(mystiko, args, pretty_json).await
        }
        AccountCommands::Import(args) => {
            execute_account_import_command(mystiko, args, pretty_json).await
        }
        AccountCommands::ExportSecretKey(args) => {
            execute_account_export_secret_key_command(mystiko, args).await
        }
        AccountCommands::List => execute_account_list_command(mystiko, pretty_json).await,
    }
}

pub async fn execute_account_create_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: AccountCreateCommand,
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
    let options = CreateAccountOptions::builder()
        .wallet_password(args.password)
        .name(args.name)
        .build();
    print_account(mystiko.accounts.create(&options).await?, pretty_json)
}

pub async fn execute_account_import_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: AccountImportCommand,
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
    let options = CreateAccountOptions::builder()
        .wallet_password(args.password)
        .name(args.name)
        .secret_key(args.secret_key)
        .build();
    print_account(mystiko.accounts.create(&options).await?, pretty_json)
}

pub async fn execute_account_export_secret_key_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: AccountExportSecretKeyCommand,
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
    let secret_key = mystiko
        .accounts
        .export_secret_key_by_shielded_address(&args.password, &args.shielded_address)
        .await?;
    println!("{}", secret_key);
    Ok(())
}

pub async fn execute_account_list_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
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
    for account in mystiko.accounts.find_all().await?.into_iter() {
        print_account(account, pretty_json)?;
    }
    Ok(())
}

fn print_account(account: Account, pretty_json: bool) -> Result<(), MystikoCliError> {
    if pretty_json {
        println!("{}", serde_json::to_string_pretty(&account)?);
    } else {
        println!("{}", serde_json::to_string(&account)?);
    }
    Ok(())
}
