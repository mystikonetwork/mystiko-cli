use clap::{Args, Subcommand};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::handler::v1::{CreateDepositOptions, QuoteDepositOptions};
use mystiko_protos::core::v1::DepositStatus;

#[derive(Debug, Clone, Args)]
pub struct DepositCommand {
    #[command(subcommand)]
    pub commands: DepositCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum DepositCommands {
    #[command(about = "get the deposit quote")]
    Quote(DepositQuoteCommand),
    #[command(about = "create a new deposit")]
    Create(DepositCreateCommand),
    #[command(about = "list all deposits")]
    List(DepositListCommand),
}

#[derive(Debug, Clone, Args)]
pub struct DepositQuoteCommand {
    #[arg(long, default_value_t = 1, help = "chain_id for the deposit")]
    pub chain_id: u64,
    #[arg(long, default_value = "ETH", help = "asset_symbol for the deposit")]
    pub asset_symbol: String,
    #[arg(long, help = "dst_chain_id for the cross-chain deposit")]
    pub dst_chain_id: Option<u64>,
    #[arg(long, help = "bridge_type for the cross-chain deposit")]
    pub bridge_type: Option<String>,
    #[arg(long, help = "timeout in milliseconds for the querying provider")]
    pub query_timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Args)]
pub struct DepositCreateCommand {
    #[arg(long, default_value_t = 1, help = "chain_id for the deposit")]
    pub chain_id: u64,
    #[arg(long, default_value = "ETH", help = "asset_symbol for the deposit")]
    pub asset_symbol: String,
    #[arg(long, help = "amount of tokens to deposit")]
    pub amount: f64,
    #[arg(long, help = "shielded_address for the deposit")]
    pub shielded_address: String,
    #[arg(long, help = "rollup_fee_amount for the deposit")]
    pub rollup_fee_amount: f64,
    #[arg(long, help = "private key for signing the transaction")]
    pub private_key: String,
    #[arg(long, help = "dst_chain_id for the cross-chain deposit")]
    pub dst_chain_id: Option<u64>,
    #[arg(long, help = "bridge_fee_amount for the cross-chain deposit")]
    pub bridge_fee_amount: Option<f64>,
    #[arg(long, help = "executor_fee_amount for the cross-chain deposit")]
    pub executor_fee_amount: Option<f64>,
    #[arg(long, help = "bridge_type for the cross-chain deposit")]
    pub bridge_type: Option<String>,
    #[arg(long, help = "timeout in milliseconds for the querying provider")]
    pub query_timeout_ms: Option<u64>,
    #[arg(
        long,
        help = "number of confirmations to wait for the asset approve transaction"
    )]
    pub asset_approve_confirmations: Option<u64>,
    #[arg(
        long,
        help = "number of confirmations to wait for the deposit transaction"
    )]
    pub deposit_confirmations: Option<u64>,
    #[arg(long, help = "timeout in milliseconds for sending a transaction")]
    pub tx_send_timeout_ms: Option<u64>,
    #[arg(long, help = "timeout in milliseconds for waiting a transaction")]
    pub tx_wait_timeout_ms: Option<u64>,
    #[arg(
        long,
        help = "interval in milliseconds for polling a transaction receipt"
    )]
    pub tx_wait_interval_ms: Option<u64>,
}

#[derive(Debug, Clone, Args)]
pub struct DepositListCommand {
    #[arg(long, help = "listing deposit(s) with the given chain_id(s)")]
    pub chain_id: Option<Vec<u64>>,
    #[arg(
        long,
        help = "listing deposit(s) with the given deposit_contract_address(es)"
    )]
    pub deposit_contract_address: Option<Vec<String>>,
    #[arg(
        long,
        help = "listing deposit(s) with the given pool_contract_address(es)"
    )]
    pub pool_contract_address: Option<Vec<String>>,
    #[arg(long, help = "listing deposit(s) with the given dst_chain_id(s)")]
    pub dst_chain_id: Option<Vec<u64>>,
    #[arg(
        long,
        help = "listing deposit(s) with the given dst_deposit_contract_address(es)"
    )]
    pub dst_deposit_contract_address: Option<Vec<String>>,
    #[arg(
        long,
        help = "listing deposit(s) with the given dst_pool_contract_address(es)"
    )]
    pub dst_pool_contract_address: Option<Vec<String>>,
    #[arg(long, help = "listing deposit(s) with the given asset_symbol(s)")]
    pub asset_symbol: Option<Vec<String>>,
    #[arg(long, help = "listing deposit(s) with the given bridge_type(s)")]
    pub bridge_type: Option<Vec<String>>,
    #[arg(long, help = "listing deposit(s) with the given status(es)")]
    pub status: Option<Vec<String>>,
    #[arg(long, help = "listing deposit(s) with the given commitment_hash(es)")]
    pub commitment_hash: Option<Vec<String>>,
    #[arg(long, help = "listing deposit(s) with the given shielded_address(es)")]
    pub shielded_address: Option<Vec<String>>,
    #[arg(long, default_value_t = 10, help = "limiting the number of deposit(s)")]
    pub limit: u64,
    #[arg(
        long,
        default_value_t = 1,
        help = "the page index of the listing deposit(s)"
    )]
    pub page: u64,
}

impl From<DepositQuoteCommand> for QuoteDepositOptions {
    fn from(args: DepositQuoteCommand) -> Self {
        let bridge_type = parse_bridge_type(args.bridge_type);
        QuoteDepositOptions::builder()
            .chain_id(args.chain_id)
            .asset_symbol(args.asset_symbol)
            .dst_chain_id(args.dst_chain_id)
            .bridge_type(bridge_type)
            .query_timeout_ms(args.query_timeout_ms)
            .build()
    }
}

impl From<DepositCreateCommand> for CreateDepositOptions {
    fn from(args: DepositCreateCommand) -> Self {
        let bridge_type = parse_bridge_type(args.bridge_type);
        CreateDepositOptions::builder()
            .chain_id(args.chain_id)
            .asset_symbol(args.asset_symbol)
            .amount(args.amount)
            .rollup_fee_amount(args.rollup_fee_amount)
            .shielded_address(args.shielded_address)
            .dst_chain_id(args.dst_chain_id)
            .bridge_type(bridge_type)
            .bridge_fee_amount(args.bridge_fee_amount)
            .executor_fee_amount(args.executor_fee_amount)
            .query_timeout_ms(args.query_timeout_ms)
            .build()
    }
}

pub(crate) fn parse_bridge_types(bridge_types: &[String]) -> Vec<i32> {
    bridge_types
        .iter()
        .filter_map(|bridge_type| parse_bridge_type(Some(bridge_type.clone())))
        .collect()
}

pub(crate) fn parse_bridge_type(bridge_type: Option<String>) -> Option<i32> {
    bridge_type.and_then(|bridge_type| {
        let bridge_type_str = format!("BRIDGE_TYPE_{}", bridge_type.to_ascii_uppercase());
        BridgeType::from_str_name(&bridge_type_str).map(|bridge_type| bridge_type as i32)
    })
}

pub(crate) fn parse_deposit_statuses(statuses: &[String]) -> Vec<i32> {
    statuses
        .iter()
        .filter_map(|status| parse_deposit_status(status))
        .collect()
}

pub(crate) fn parse_deposit_status(status: &str) -> Option<i32> {
    let status_str = format!("DEPOSIT_STATUS_{}", status.to_ascii_uppercase());
    DepositStatus::from_str_name(&status_str).map(|status| status as i32)
}
