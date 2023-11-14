use crate::{
    print_json, AccountCommand, AccountCommands, AccountCreateCommand,
    AccountExportSecretKeyCommand, AccountImportCommand, MystikoCliError,
};
use mystiko_core::{AccountHandler, Mystiko};
use mystiko_protos::core::document::v1::Account;
use mystiko_protos::core::handler::v1::{CreateAccountOptions, UpdateAccountOptions};
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_account_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: AccountCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<A::Error>,
{
    match args.commands {
        AccountCommands::Create(args) => {
            execute_account_create_command(mystiko, args, compact_json).await
        }
        AccountCommands::Import(args) => {
            execute_account_import_command(mystiko, args, compact_json).await
        }
        AccountCommands::ExportSecretKey(args) => {
            execute_account_export_secret_key_command(mystiko, args).await
        }
        AccountCommands::List => execute_account_list_command(mystiko, compact_json).await,
    }
}

pub async fn execute_account_create_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: AccountCreateCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<A::Error>,
{
    let options = CreateAccountOptions::builder()
        .wallet_password(args.password)
        .name(args.name)
        .build();
    let account = mystiko.accounts.create(&options).await?;
    print_json(&account, compact_json)
}

pub async fn execute_account_import_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: AccountImportCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<A::Error>,
{
    let options = CreateAccountOptions::builder()
        .wallet_password(args.password)
        .name(args.name)
        .secret_key(args.secret_key)
        .build();
    let account = mystiko.accounts.create(&options).await?;
    print_json(&account, compact_json)
}

pub async fn execute_account_export_secret_key_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    args: AccountExportSecretKeyCommand,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<A::Error>,
{
    let secret_key = mystiko
        .accounts
        .export_secret_key_by_shielded_address(&args.password, &args.shielded_address)
        .await?;
    println!("{}", secret_key);
    Ok(())
}

pub async fn execute_account_list_command<F, S, W, A, D, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, Y, R>,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<A::Error>,
{
    for account in mystiko.accounts.find_all().await?.into_iter() {
        print_json(&account, compact_json)?;
    }
    Ok(())
}
