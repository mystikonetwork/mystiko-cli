use crate::{
    print_json, MystikoCliError, WalletCommand, WalletCommands, WalletCreateCommand,
    WalletExportMnemonicPhraseCommand, WalletImportCommand, WalletUpdatePasswordCommand,
};
use anyhow::anyhow;
use mystiko_core::{AccountHandler, Mystiko, WalletHandler};
use mystiko_protos::core::document::v1::{Account, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateWalletOptions, MnemonicOptions, UpdateAccountOptions,
};
use mystiko_protos::core::v1::MnemonicType;
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
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<W::Error> + From<A::Error>,
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
        WalletCommands::UpdatePassword(args) => {
            execute_wallet_update_password_command(mystiko, args, compact_json).await
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
    let words = args.mnemonic.split_whitespace().collect::<Vec<&str>>();
    let mnemonic = if words.len() == 12 {
        MnemonicOptions::builder()
            .mnemonic_phrase(args.mnemonic)
            .mnemonic_type(MnemonicType::Web)
            .build()
    } else if words.len() == 24 {
        MnemonicOptions::builder()
            .mnemonic_phrase(args.mnemonic)
            .mnemonic_type(MnemonicType::Rust)
            .build()
    } else {
        return Err(MystikoCliError::AnyhowError(anyhow!(
            "Invalid mnemonic phrase"
        )));
    };
    let options = CreateWalletOptions::builder()
        .password(args.password)
        .mnemonic(mnemonic)
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

pub async fn execute_wallet_update_password_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: WalletUpdatePasswordCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    MystikoCliError: From<W::Error> + From<A::Error>,
{
    let wallet = mystiko
        .wallets
        .update_password(&args.old, &args.new)
        .await?;
    mystiko
        .accounts
        .update_encryption(&args.old, &args.new)
        .await?;
    print_json(&wallet, compact_json)
}
