use clap::{Args, Subcommand};

#[derive(Debug, Clone, Args)]
pub struct AccountCommand {
    #[command(subcommand)]
    pub commands: AccountCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum AccountCommands {
    #[command(about = "create a new account")]
    Create(AccountCreateCommand),
    #[command(about = "import an account")]
    Import(AccountImportCommand),
    #[command(about = "get an account")]
    ExportSecretKey(AccountExportSecretKeyCommand),
    #[command(about = "list all accounts")]
    List,
}

#[derive(Debug, Clone, Args)]
pub struct AccountCreateCommand {
    #[arg(long, help = "password for wallet")]
    pub password: String,
    #[arg(long, help = "name for this account")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct AccountImportCommand {
    #[arg(long, help = "password for wallet")]
    pub password: String,
    #[arg(long, help = "secret key for this account")]
    pub secret_key: String,
    #[arg(long, help = "name for this account")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct AccountExportSecretKeyCommand {
    #[arg(long, help = "password for wallet")]
    pub password: String,
    #[arg(long, help = "shielded address of the account")]
    pub shielded_address: String,
}
