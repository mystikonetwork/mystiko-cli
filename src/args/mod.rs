mod account;
mod deposit;
mod scanner;
mod spend;
mod synchronizer;
mod wallet;

pub use account::*;
pub use deposit::*;
pub use scanner::*;
pub use spend::*;
pub use synchronizer::*;
pub use wallet::*;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[command(version, about)]
pub struct MystikoCliArgs {
    #[arg(long, help = "enable testnet mode")]
    pub testnet: bool,
    #[arg(long, help = "enable staging mode")]
    pub staging: bool,
    #[arg(long, help = "the git revision of the config file")]
    pub config_git_revision: Option<String>,
    #[arg(long, help = "path to config file")]
    pub config_path: Option<String>,
    #[arg(long, help = "logging level of current crate", default_value = "info")]
    pub logging_level: String,
    #[arg(
        long,
        help = "logging level of external crates",
        default_value = "warn"
    )]
    pub extern_logging_level: String,
    #[arg(long, help = "path to database file")]
    pub db_path: Option<String>,
    #[arg(long, help = "path to static cache file")]
    pub static_cache_path: Option<String>,
    #[arg(long, help = "in memory database")]
    pub in_memory: bool,
    #[arg(long, help = "output compact json string")]
    pub compact_json: bool,
    #[command(subcommand)]
    pub commands: MystikoCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum MystikoCommands {
    #[command(about = "wallet command for managing the mystiko wallet")]
    Wallet(WalletCommand),
    #[command(about = "account command for managing accounts")]
    Account(AccountCommand),
    #[command(about = "deposit command for managing deposits")]
    Deposit(DepositCommand),
    #[command(about = "spend command for managing spends")]
    Spend(SpendCommand),
    #[command(about = "scanner command for scanning the private assets")]
    Scanner(ScannerCommand),
    #[command(about = "synchronizer command for synchronizing data")]
    Synchronizer(SynchronizerCommand),
}
