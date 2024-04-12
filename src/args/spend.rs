use crate::args::parse_bridge_type;
use clap::{Args, Subcommand};
use mystiko_protos::core::handler::v1::{CreateSpendOptions, QuoteSpendOptions};
use mystiko_protos::core::v1::{SpendStatus, SpendType};

#[derive(Debug, Clone, Args)]
pub struct SpendCommand {
    #[command(subcommand)]
    pub commands: SpendCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SpendCommands {
    #[command(about = "get the spend quote")]
    Quote(SpendQuoteCommand),
    #[command(about = "create a new spend")]
    Create(SpendCreateCommand),
    #[command(about = "list all spends")]
    List(SpendListCommand),
}

#[derive(Debug, Clone, Args)]
pub struct SpendQuoteCommand {
    #[arg(long, default_value_t = 1, help = "chain_id for the spend")]
    pub chain_id: u64,
    #[arg(long, default_value = "ETH", help = "asset_symbol for the spend")]
    pub asset_symbol: String,
    #[arg(long, help = "type of the spend, default value is withdraw")]
    pub spend_type: Option<String>,
    #[arg(long, help = "bridge_type for spending cross-chain assets")]
    pub bridge_type: Option<String>,
    #[arg(long, help = "version number of the assets pool")]
    pub version: Option<u32>,
    #[arg(long, help = "amount of assets to spend")]
    pub amount: Option<f64>,
    #[arg(
        long,
        default_value_t = false,
        help = "quote spend with relayer(s) fee"
    )]
    pub use_relayer: bool,
    #[arg(long, help = "timeout in milliseconds for the querying provider")]
    pub query_timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Args)]
pub struct SpendCreateCommand {
    #[arg(long, default_value_t = 1, help = "chain_id for the spend")]
    pub chain_id: u64,
    #[arg(long, default_value = "ETH", help = "asset_symbol for the spend")]
    pub asset_symbol: String,
    #[arg(long, help = "type of the spend, default value is withdraw")]
    pub spend_type: Option<String>,
    #[arg(long, help = "bridge_type for spending cross-chain assets")]
    pub bridge_type: Option<String>,
    #[arg(long, help = "version number of the assets pool")]
    pub version: Option<u32>,
    #[arg(long, help = "recipient address for the spend")]
    pub recipient: String,
    #[arg(long, help = "password for the current wallet")]
    pub password: String,
    #[arg(long, help = "private key for signing the transaction")]
    pub private_key: Option<String>,
    #[arg(long, help = "provider for signing and sending the transaction")]
    pub signer_provider: Option<String>,
    #[arg(long, help = "amount of assets to spend")]
    pub amount: f64,
    #[arg(long, help = "rollup fee for the spend output commitments")]
    pub rollup_fee: Option<f64>,
    #[arg(long, help = "name of relayer to relaying the spend transaction")]
    pub relayer: Option<String>,
    #[arg(long, help = "timeout in milliseconds for the querying provider")]
    pub query_timeout_ms: Option<u64>,
    #[arg(
        long,
        help = "number of confirmations to wait for the spend transaction"
    )]
    pub confirmations: Option<u64>,
    #[arg(long, help = "timeout in milliseconds for sending a transaction")]
    pub tx_send_timeout_ms: Option<u64>,
    #[arg(long, help = "timeout in milliseconds for waiting a transaction")]
    pub tx_wait_timeout_ms: Option<u64>,
    #[arg(
        long,
        help = "interval in milliseconds for polling a transaction receipt"
    )]
    pub tx_wait_interval_ms: Option<u64>,
    #[arg(
        long,
        help = "interval in milliseconds for polling a relayer for the spend transaction"
    )]
    pub relayer_wait_interval_ms: Option<u64>,
    #[arg(
        long,
        help = "timeout in milliseconds for polling a relayer for the spend transaction"
    )]
    pub relayer_wait_timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Args)]
pub struct SpendListCommand {
    #[arg(long, help = "listing spend(s) with the given chain_id(s)")]
    pub chain_id: Option<Vec<u64>>,
    #[arg(long, help = "listing spend(s) with the given contract_address(es)")]
    pub contract_address: Option<Vec<String>>,
    #[arg(long, help = "listing spend(s) with the given asset_symbol(s)")]
    pub asset_symbol: Option<Vec<String>>,
    #[arg(long, help = "listing spend(s) with the given bridge_type(s)")]
    pub bridge_type: Option<Vec<String>>,
    #[arg(long, help = "listing spend(s) with the given spend_type(s)")]
    pub spend_type: Option<Vec<String>>,
    #[arg(long, help = "listing spend(s) with the given status(es)")]
    pub status: Option<Vec<String>>,
    #[arg(long, help = "listing spend(s) with the given recipient(s)")]
    pub recipient: Option<Vec<String>>,
    #[arg(long, help = "listing spend(s) with the given relayer address(es)")]
    pub relayer: Option<Vec<String>>,
    #[arg(long, default_value_t = 10, help = "limiting the number of spend(s)")]
    pub limit: u64,
    #[arg(
        long,
        default_value_t = 1,
        help = "the page index of the listing spend(s)"
    )]
    pub page: u64,
}

impl From<SpendQuoteCommand> for QuoteSpendOptions {
    fn from(args: SpendQuoteCommand) -> Self {
        QuoteSpendOptions::builder()
            .chain_id(args.chain_id)
            .asset_symbol(args.asset_symbol)
            .spend_type(parse_spend_type(args.spend_type))
            .bridge_type(parse_bridge_type(args.bridge_type))
            .version(args.version)
            .amount(args.amount)
            .query_timeout_ms(args.query_timeout_ms)
            .use_relayer(args.use_relayer)
            .build()
    }
}

impl From<SpendCreateCommand> for CreateSpendOptions {
    fn from(args: SpendCreateCommand) -> Self {
        CreateSpendOptions::builder()
            .chain_id(args.chain_id)
            .asset_symbol(args.asset_symbol)
            .spend_type(parse_spend_type(args.spend_type))
            .bridge_type(parse_bridge_type(args.bridge_type))
            .version(args.version)
            .recipient(args.recipient)
            .wallet_password(args.password)
            .amount(args.amount)
            .rollup_fee_amount(args.rollup_fee)
            .gas_relayer(args.relayer)
            .query_timeout_ms(args.query_timeout_ms)
            .build()
    }
}

pub(crate) fn parse_spend_types(spend_types: &[String]) -> Vec<i32> {
    spend_types
        .iter()
        .map(|spend_type| parse_spend_type(Some(spend_type.to_string())))
        .collect::<Vec<_>>()
}

pub(crate) fn parse_spend_type(spend_type: Option<String>) -> i32 {
    spend_type
        .and_then(|spend_type| {
            let spend_type = format!("SPEND_TYPE_{}", spend_type.to_uppercase());
            SpendType::from_str_name(&spend_type).map(|spend_type| spend_type as i32)
        })
        .unwrap_or(SpendType::Withdraw as i32)
}

pub(crate) fn parse_spend_statuses(statues: &[String]) -> Vec<i32> {
    statues
        .iter()
        .filter_map(|status| parse_spend_status(status))
        .collect::<Vec<_>>()
}

pub(crate) fn parse_spend_status(status: &str) -> Option<i32> {
    let status = format!("SPEND_STATUS_{}", status.to_uppercase());
    SpendStatus::from_str_name(&status).map(|status| status as i32)
}
