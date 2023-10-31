use clap::{Args, Subcommand};

#[derive(Debug, Clone, Args)]
pub struct WalletCommand {
    #[command(subcommand)]
    pub commands: WalletCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum WalletCommands {
    #[command(about = "create a new wallet")]
    Create(WalletCreateCommand),
    #[command(about = "import a wallet")]
    Import(WalletImportCommand),
    #[command(about = "get the current wallet")]
    ExportMnemonic(WalletExportMnemonicPhraseCommand),
}

#[derive(Debug, Clone, Args)]
pub struct WalletCreateCommand {
    #[arg(long, help = "password for wallet")]
    pub password: String,
}

#[derive(Debug, Clone, Args)]
pub struct WalletImportCommand {
    #[arg(long, help = "password for wallet")]
    pub password: String,
    #[arg(long, help = "mnemonic phrases for wallet")]
    pub mnemonic: String,
}

#[derive(Debug, Clone, Args)]
pub struct WalletExportMnemonicPhraseCommand {
    #[arg(long, help = "password for wallet")]
    pub password: String,
}
