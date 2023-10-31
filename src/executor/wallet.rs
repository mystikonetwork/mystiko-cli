use crate::{
    MystikoCliError, WalletCommand, WalletCommands, WalletCreateCommand,
    WalletExportMnemonicPhraseCommand, WalletImportCommand,
};
use mystiko_core::Mystiko;
use mystiko_protos::core::document::v1::Wallet;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_wallet_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: WalletCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
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

pub async fn execute_wallet_create_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: WalletCreateCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
{
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .build();
    print_wallet(mystiko.wallets.create(&options).await?, pretty_json)
}

pub async fn execute_wallet_import_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: WalletImportCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
{
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .mnemonic_phrase(args.mnemonic)
        .build();
    print_wallet(mystiko.wallets.create(&options).await?, pretty_json)
}

pub async fn execute_wallet_export_mnemonic_phrase_command<F, S>(
    mystiko: &Mystiko<F, S>,
    args: WalletExportMnemonicPhraseCommand,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
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
