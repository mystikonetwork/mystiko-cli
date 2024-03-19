use clap::{Args, Subcommand};
use mystiko_protos::core::synchronizer::v1::{
    ResetChainOptions, SyncOptions, SynchronizerResetOptions,
};

#[derive(Debug, Clone, Args)]
pub struct SynchronizerCommand {
    #[command(subcommand)]
    pub commands: SynchronizerCommands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum SynchronizerCommands {
    #[command(about = "sync data from chain")]
    Sync(SynchronizerSyncCommand),
    #[command(about = "get synchronizer status")]
    Status(SynchronizerStatusCommand),
    #[command(about = "reset synchronizer data")]
    Reset(SynchronizerResetCommand),
}

#[derive(Debug, Clone, Args)]
pub struct SynchronizerSyncCommand {
    #[arg(
        long,
        default_value_t = false,
        help = "disable fetching from datapacker"
    )]
    pub disable_datapacker_fetcher: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "enable validating data fetched from datapacker"
    )]
    pub enable_datapacker_fetcher_validate: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "disable fetching from sequencer"
    )]
    pub disable_sequencer_fetcher: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "enable validating data fetched from sequencer"
    )]
    pub enable_sequencer_fetcher_validate: bool,
    #[arg(long, default_value_t = false, help = "disable fetching from provider")]
    pub disable_provider_fetcher: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "disable validating data fetched from provider"
    )]
    pub disable_provider_fetcher_validate: bool,
    #[arg(long, default_value_t = false, help = "disable rule validator")]
    pub disable_rule_validator: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "disable rule validator integrity check"
    )]
    pub disable_rule_validator_integrity_check: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "disable rule validator sequence check"
    )]
    pub disable_rule_validator_sequence_check: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "disable rule validator counter check"
    )]
    pub disable_rule_validator_counter_check: bool,
    #[arg(
        long,
        default_value_t = false,
        help = "disable rule validator tree check"
    )]
    pub disable_rule_validator_tree_check: bool,
    #[arg(long, help = "fetcher fetch timeout in milliseconds")]
    pub fetcher_fetch_timeout_ms: Option<u64>,
    #[arg(long, help = "fetcher query_loaded_block timeout in milliseconds")]
    pub fetcher_query_loaded_block_timeout_ms: Option<u64>,
    #[arg(long, help = "fetcher validate concurrency")]
    pub validator_validate_concurrency: Option<u64>,
    #[arg(long, help = "chains to be synchronized")]
    pub chain_id: Vec<u64>,
}

#[derive(Debug, Clone, Args)]
pub struct SynchronizerStatusCommand {
    #[arg(long, default_value_t = false, help = "include contracts status")]
    pub with_contracts: bool,
}

#[derive(Debug, Clone, Args)]
pub struct SynchronizerResetCommand {
    #[arg(long, help = "reset the synchronized data of the specific chain")]
    pub chain_id: u64,
    #[arg(long, help = "reset the synchronized data of the specific contract(s)")]
    pub contract_address: Option<Vec<String>>,
    #[arg(
        long,
        help = "reset the synchronized data to the given block number, \
        if not specified, reset to contract deploy block"
    )]
    pub to: Option<u64>,
}

impl From<SynchronizerSyncCommand> for SyncOptions {
    fn from(command: SynchronizerSyncCommand) -> Self {
        SyncOptions::builder()
            .disable_datapacker_fetcher(command.disable_datapacker_fetcher)
            .enable_datapacker_fetcher_validate(command.enable_datapacker_fetcher_validate)
            .disable_sequencer_fetcher(command.disable_sequencer_fetcher)
            .enable_sequencer_fetcher_validate(command.enable_sequencer_fetcher_validate)
            .disable_provider_fetcher(command.disable_provider_fetcher)
            .disable_provider_fetcher_validate(command.disable_provider_fetcher_validate)
            .disable_rule_validator(command.disable_rule_validator)
            .disable_rule_validator_integrity_check(command.disable_rule_validator_integrity_check)
            .disable_rule_validator_sequence_check(command.disable_rule_validator_sequence_check)
            .disable_rule_validator_counter_check(command.disable_rule_validator_counter_check)
            .disable_rule_validator_tree_check(command.disable_rule_validator_tree_check)
            .fetcher_fetch_timeout_ms(command.fetcher_fetch_timeout_ms)
            .fetcher_query_loaded_block_timeout_ms(command.fetcher_query_loaded_block_timeout_ms)
            .validator_validate_concurrency(command.validator_validate_concurrency)
            .chain_ids(command.chain_id)
            .build()
    }
}

impl From<SynchronizerResetCommand> for SynchronizerResetOptions {
    fn from(args: SynchronizerResetCommand) -> Self {
        let chain_options = ResetChainOptions::builder()
            .chain_id(args.chain_id)
            .contract_addresses(args.contract_address.unwrap_or_default())
            .block_number(args.to)
            .build();
        SynchronizerResetOptions::builder()
            .chains(vec![chain_options])
            .build()
    }
}
