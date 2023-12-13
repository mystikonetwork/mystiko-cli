use crate::{
    print_json, MystikoCliError, WalletCommand, WalletCommands, WalletCreateCommand,
    WalletExportMnemonicPhraseCommand, WalletImportCommand,
};
use mystiko_core::{Mystiko, WalletHandler};
use mystiko_protos::core::document::v1::Wallet;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_wallet_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: WalletCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    MystikoCliError: From<W::Error>,
{
    match args.commands {
        WalletCommands::Create(args) => {
            execute_wallet_create_command(mystiko, args, compact_json).await
        }
        WalletCommands::Import(args) => {
            execute_wallet_import_command(mystiko, args, compact_json).await
        }
        WalletCommands::ExportMnemonic(args) => {
            execute_wallet_export_mnemonic_phrase_command(mystiko, args).await
        }
    }
}

pub async fn execute_wallet_create_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: WalletCreateCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    MystikoCliError: From<W::Error>,
{
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .build();
    let wallet = mystiko.wallets.create(&options).await?;
    print_json(&wallet, compact_json)
}

pub async fn execute_wallet_import_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: WalletImportCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    MystikoCliError: From<W::Error>,
{
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .mnemonic_phrase(args.mnemonic)
        .build();
    let wallet = mystiko.wallets.create(&options).await?;
    print_json(&wallet, compact_json)
}

pub async fn execute_wallet_export_mnemonic_phrase_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: WalletExportMnemonicPhraseCommand,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    MystikoCliError: From<W::Error>,
{
    let mnemonic_phrase = mystiko
        .wallets
        .export_mnemonic_phrase(&args.password)
        .await?;
    println!("{}", mnemonic_phrase);
    Ok(())
}
