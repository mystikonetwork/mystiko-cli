use crate::args::parse_bridge_types;
use clap::{Args, Subcommand};
use mystiko_protos::core::scanner::v1::{
    AssetChainImportOptions, AssetImportOptions, AssetsOptions, BalanceOptions,
    ScannerResetOptions, ScannerScanOptions, ScannerSyncOptions,
};

#[derive(Debug, Clone, Args)]
pub struct ScannerCommand {
    #[command(subcommand)]
    pub commands: ScannerCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum ScannerCommands {
    #[command(about = "sync the chain for available commitments")]
    Sync(ScannerSyncCommand),
    #[command(about = "scan the chain for available private assets")]
    Scan(ScannerScanCommand),
    #[command(about = "reset the scanner")]
    Reset(ScannerResetCommand),
    #[command(about = "import the assets from the given transaction(s)")]
    Import(ScannerImportCommand),
    #[command(about = "get the balance of private assets in the current wallet")]
    Balance(ScannerBalanceCommand),
    #[command(about = "group the private assets by chain_id/bridge_type/asset_symbol")]
    Assets(ScannerAssetsCommand),
}

#[derive(Debug, Clone, Args)]
pub struct ScannerSyncCommand {
    #[arg(long, help = "password of the current wallet")]
    pub password: String,
    #[arg(
        long,
        default_value_t = 1,
        help = "number of concurrency for each sync round"
    )]
    pub concurrency: u32,
}

#[derive(Debug, Clone, Args)]
pub struct ScannerScanCommand {
    #[arg(long, help = "password of the current wallet")]
    pub password: String,
    #[arg(
        long,
        default_value_t = 10000,
        help = "batch size for each scanning round"
    )]
    pub batch_size: u64,
    #[arg(
        long,
        default_value_t = 1,
        help = "number of concurrency for each scanning round"
    )]
    pub concurrency: u32,
    #[arg(long, help = "the shielded address(es) to be scanned for")]
    pub shielded_address: Option<Vec<String>>,
}

#[derive(Debug, Clone, Args)]
pub struct ScannerResetCommand {
    #[arg(long, help = "reset the scanner to the given commitment id")]
    pub to_id: Option<String>,
    #[arg(long, help = "the shielded address(es) to be reset")]
    pub shielded_address: Option<Vec<String>>,
}

#[derive(Debug, Clone, Args)]
pub struct ScannerImportCommand {
    #[arg(long, help = "password of the current wallet")]
    pub password: String,
    #[arg(long, help = "the chain id to be imported")]
    pub chain_id: u64,
    #[arg(long, help = "the transaction hash(es) to be imported")]
    pub tx_hashes: Vec<String>,
}

#[derive(Debug, Clone, Args)]
pub struct ScannerBalanceCommand {
    #[arg(long, help = "to include the spent total in the balance")]
    pub with_spent: bool,
    #[arg(long, help = "show the balance of the given shielded address(es)")]
    pub shielded_address: Option<Vec<String>>,
    #[arg(long, help = "show the balance of the given chain id(s)")]
    pub chain_id: Option<Vec<u64>>,
    #[arg(long, help = "show the balance of the given contract address(es)")]
    pub contract_address: Option<Vec<String>>,
    #[arg(long, help = "show the balance of the given asset symbol(s)")]
    pub asset_symbol: Option<Vec<String>>,
    #[arg(long, help = "show the balance of the given bridge type(s)")]
    pub bridge_type: Option<Vec<String>>,
}

#[derive(Debug, Clone, Args)]
pub struct ScannerAssetsCommand {
    #[arg(long, help = "show the assets of the given chain_id(s)")]
    pub chain_id: Option<Vec<u64>>,
    #[arg(long, help = "show the assets of the given contract address(es)")]
    pub shielded_address: Option<Vec<String>>,
}

impl From<ScannerSyncCommand> for ScannerSyncOptions {
    fn from(args: ScannerSyncCommand) -> Self {
        ScannerSyncOptions::builder()
            .wallet_password(args.password)
            .concurrency(args.concurrency)
            .build()
    }
}

impl From<ScannerScanCommand> for ScannerScanOptions {
    fn from(args: ScannerScanCommand) -> Self {
        ScannerScanOptions::builder()
            .wallet_password(args.password)
            .batch_size(args.batch_size)
            .concurrency(args.concurrency)
            .shielded_addresses(args.shielded_address.unwrap_or_default())
            .build()
    }
}

impl From<ScannerResetCommand> for ScannerResetOptions {
    fn from(args: ScannerResetCommand) -> Self {
        ScannerResetOptions::builder()
            .reset_to_id(args.to_id.unwrap_or_default())
            .shielded_addresses(args.shielded_address.unwrap_or_default())
            .build()
    }
}

impl From<ScannerImportCommand> for AssetImportOptions {
    fn from(args: ScannerImportCommand) -> Self {
        AssetImportOptions::builder()
            .wallet_password(args.password)
            .chains([AssetChainImportOptions::builder()
                .chain_id(args.chain_id)
                .tx_hashes(args.tx_hashes)
                .build()])
            .build()
    }
}

impl From<ScannerBalanceCommand> for BalanceOptions {
    fn from(args: ScannerBalanceCommand) -> Self {
        BalanceOptions::builder()
            .with_spent(args.with_spent)
            .shielded_addresses(args.shielded_address.unwrap_or_default())
            .chain_ids(args.chain_id.unwrap_or_default())
            .contract_addresses(args.contract_address.unwrap_or_default())
            .asset_symbols(args.asset_symbol.unwrap_or_default())
            .bridge_types(parse_bridge_types(&args.bridge_type.unwrap_or_default()))
            .build()
    }
}

impl From<ScannerAssetsCommand> for AssetsOptions {
    fn from(args: ScannerAssetsCommand) -> Self {
        AssetsOptions::builder()
            .shielded_addresses(args.shielded_address.unwrap_or_default())
            .build()
    }
}
