use crate::{
    AccountCommand, AccountCommands, AccountCreateCommand, AccountExportSecretKeyCommand,
    AccountImportCommand, MystikoCliError,
};
use mystiko_core::Mystiko;
use mystiko_protos::core::document::v1::Account;
use mystiko_protos::core::handler::v1::CreateAccountOptions;
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_account_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: AccountCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
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

pub async fn execute_account_create_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: AccountCreateCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
{
    let options = CreateAccountOptions::builder()
        .wallet_password(args.password)
        .name(args.name)
        .build();
    print_account(mystiko.accounts.create(&options).await?, pretty_json)
}

pub async fn execute_account_import_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: AccountImportCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
{
    let options = CreateAccountOptions::builder()
        .wallet_password(args.password)
        .name(args.name)
        .secret_key(args.secret_key)
        .build();
    print_account(mystiko.accounts.create(&options).await?, pretty_json)
}

pub async fn execute_account_export_secret_key_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: AccountExportSecretKeyCommand,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
{
    let secret_key = mystiko
        .accounts
        .export_secret_key_by_shielded_address(&args.password, &args.shielded_address)
        .await?;
    println!("{}", secret_key);
    Ok(())
}

pub async fn execute_account_list_command<F, S>(
    mystiko: &Mystiko<F, S>,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
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
