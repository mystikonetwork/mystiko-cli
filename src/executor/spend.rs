use crate::args::{parse_bridge_types, parse_spend_statuses, parse_spend_types};
use crate::executor::create_list_filter;
use crate::{
    print_json, MystikoCliError, SpendCommand, SpendCommands, SpendCreateCommand, SpendListCommand,
    SpendQuoteCommand,
};
use mystiko_core::{Mystiko, SpendColumn, SpendHandler};
use mystiko_protos::core::document::v1::Spend;
use mystiko_protos::core::handler::v1::{
    CreateSpendOptions, QuoteSpendOptions, SendSpendOptions, SpendQuote, SpendSummary,
};
use mystiko_protos::storage::v1::SubFilter;
use mystiko_storage::{StatementFormatter, Storage};

pub async fn execute_spend_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SpendCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    X: SpendHandler<
        Spend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
    >,
    MystikoCliError: From<X::Error>,
{
    match args.commands {
        SpendCommands::Quote(args) => {
            execute_spend_quote_command(mystiko, args, compact_json).await
        }
        SpendCommands::Create(args) => {
            execute_spend_create_command(mystiko, args, compact_json).await
        }
        SpendCommands::List(args) => execute_spend_list_command(mystiko, args, compact_json).await,
    }
}

pub async fn execute_spend_quote_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SpendQuoteCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    X: SpendHandler<
        Spend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
    >,
    MystikoCliError: From<X::Error>,
{
    let quote = mystiko.spends.quote(args.into()).await?;
    print_json(&quote, compact_json)
}

pub async fn execute_spend_create_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SpendCreateCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    X: SpendHandler<
        Spend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
    >,
    MystikoCliError: From<X::Error>,
{
    let spend = mystiko.spends.create(args.clone().into()).await?;
    let send_options = SendSpendOptions::builder()
        .spend_id(spend.id)
        .wallet_password(args.password)
        .private_key(args.private_key)
        .query_timeout_ms(args.query_timeout_ms)
        .spend_confirmations(args.confirmations)
        .tx_wait_timeout_ms(args.tx_wait_timeout_ms)
        .tx_wait_interval_ms(args.tx_wait_interval_ms)
        .tx_send_timeout_ms(args.tx_send_timeout_ms)
        .relayer_wait_interval_ms(args.relayer_wait_interval_ms)
        .relayer_wait_timeout_ms(args.relayer_wait_timeout_ms)
        .build();
    let spend = mystiko.spends.send(send_options).await?;
    print_json(&spend, compact_json)
}

pub async fn execute_spend_list_command<F, S, W, A, D, X, Y, R>(
    mystiko: &Mystiko<F, S, W, A, D, X, Y, R>,
    args: SpendListCommand,
    compact_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    X: SpendHandler<
        Spend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
    >,
    MystikoCliError: From<X::Error>,
{
    let mut sub_filters = vec![];
    if let Some(chain_ids) = args.chain_id {
        if !chain_ids.is_empty() {
            sub_filters.push(SubFilter::in_list(SpendColumn::ChainId, chain_ids));
        }
    }
    if let Some(contract_addresses) = args.contract_address {
        if !contract_addresses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                SpendColumn::ContractAddress,
                contract_addresses,
            ));
        }
    }
    if let Some(asset_symbols) = args.asset_symbol {
        if !asset_symbols.is_empty() {
            sub_filters.push(SubFilter::in_list(SpendColumn::AssetSymbol, asset_symbols));
        }
    }
    if let Some(bridge_types) = args.bridge_type {
        if !bridge_types.is_empty() {
            sub_filters.push(SubFilter::in_list(
                SpendColumn::BridgeType,
                parse_bridge_types(&bridge_types),
            ));
        }
    }
    if let Some(spend_types) = args.spend_type {
        if !spend_types.is_empty() {
            sub_filters.push(SubFilter::in_list(
                SpendColumn::SpendType,
                parse_spend_types(&spend_types),
            ));
        }
    }
    if let Some(statuses) = args.status {
        if !statuses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                SpendColumn::Status,
                parse_spend_statuses(&statuses),
            ));
        }
    }
    if let Some(recipients) = args.recipient {
        if !recipients.is_empty() {
            sub_filters.push(SubFilter::in_list(SpendColumn::Recipient, recipients));
        }
    }
    if let Some(relayers) = args.relayer {
        if !relayers.is_empty() {
            sub_filters.push(SubFilter::in_list(SpendColumn::GasRelayerAddress, relayers));
        }
    }
    let spends = mystiko
        .spends
        .find(create_list_filter(sub_filters, args.limit, args.page))
        .await?;
    for spend in spends.into_iter() {
        print_json(&spend, compact_json)?;
    }
    Ok(())
}
