use crate::args::{parse_bridge_types, parse_deposit_statuses};
use crate::{
    DepositCommand, DepositCommands, DepositCreateCommand, DepositListCommand, DepositQuoteCommand,
    MystikoCliError,
};
use mystiko_core::{
    AccountHandler, DepositColumn, DepositHandler, Mystiko, SynchronizerHandler, WalletHandler,
};
use mystiko_protos::core::document::v1::{Account, Deposit, Wallet};
use mystiko_protos::core::handler::v1::{
    CreateAccountOptions, CreateDepositOptions, CreateWalletOptions, DepositQuote, DepositSummary,
    QuoteDepositOptions, SendDepositOptions, UpdateAccountOptions,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
use mystiko_protos::storage::v1::{
    Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter,
};
use mystiko_storage::{DocumentColumn, StatementFormatter, Storage};

pub async fn execute_deposit_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: DepositCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    D: DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    >,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<W::Error> + From<A::Error> + From<D::Error> + From<Y::Error>,
{
    match args.commands {
        DepositCommands::Quote(args) => {
            execute_deposit_quote_command(mystiko, args, pretty_json).await
        }
        DepositCommands::Create(args) => {
            execute_deposit_create_command(mystiko, args, pretty_json).await
        }
        DepositCommands::List(args) => {
            execute_deposit_list_command(mystiko, args, pretty_json).await
        }
    }
}

pub async fn execute_deposit_quote_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: DepositQuoteCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    D: DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    >,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<W::Error> + From<A::Error> + From<D::Error> + From<Y::Error>,
{
    let quote = mystiko.deposits.quote(args.into()).await?;
    if pretty_json {
        println!("{}", serde_json::to_string_pretty(&quote)?);
    } else {
        println!("{}", serde_json::to_string(&quote)?);
    }
    Ok(())
}

pub async fn execute_deposit_create_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: DepositCreateCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    D: DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    >,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<W::Error> + From<A::Error> + From<D::Error> + From<Y::Error>,
{
    let deposit = mystiko.deposits.create(args.clone().into()).await?;
    let options = SendDepositOptions::builder()
        .deposit_id(deposit.id)
        .private_key(args.private_key)
        .query_timeout_ms(args.query_timeout_ms)
        .asset_approve_confirmations(args.asset_approve_confirmations)
        .deposit_confirmations(args.deposit_confirmations)
        .tx_wait_interval_ms(args.tx_wait_interval_ms)
        .tx_wait_timeout_ms(args.tx_wait_timeout_ms)
        .tx_send_timeout_ms(args.tx_send_timeout_ms)
        .build();
    let deposit = mystiko.deposits.send(options).await?;
    print_deposit(deposit, pretty_json)
}

pub async fn execute_deposit_list_command<F, S, W, A, D, Y>(
    mystiko: &Mystiko<F, S, W, A, D, Y>,
    args: DepositListCommand,
    pretty_json: bool,
) -> Result<(), MystikoCliError>
where
    F: StatementFormatter,
    S: Storage,
    W: WalletHandler<Wallet, CreateWalletOptions>,
    A: AccountHandler<Account, CreateAccountOptions, UpdateAccountOptions>,
    D: DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
    >,
    Y: SynchronizerHandler<SyncOptions, SynchronizerStatus>,
    MystikoCliError: From<W::Error> + From<A::Error> + From<D::Error> + From<Y::Error>,
{
    let mut sub_filters = vec![];
    if let Some(chain_ids) = args.chain_id {
        if !chain_ids.is_empty() {
            sub_filters.push(SubFilter::in_list(DepositColumn::ChainId, chain_ids));
        }
    }
    if let Some(deposit_contract_addresses) = args.deposit_contract_address {
        if !deposit_contract_addresses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::ContractAddress,
                deposit_contract_addresses,
            ));
        }
    }
    if let Some(pool_contract_addresses) = args.pool_contract_address {
        if !pool_contract_addresses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::PoolAddress,
                pool_contract_addresses,
            ));
        }
    }
    if let Some(dst_chain_ids) = args.dst_chain_id {
        if !dst_chain_ids.is_empty() {
            sub_filters.push(SubFilter::in_list(DepositColumn::DstChainId, dst_chain_ids));
        }
    }
    if let Some(dst_deposit_contract_addresses) = args.dst_deposit_contract_address {
        if !dst_deposit_contract_addresses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::DstChainContractAddress,
                dst_deposit_contract_addresses,
            ));
        }
    }
    if let Some(dst_pool_contract_addresses) = args.dst_pool_contract_address {
        if !dst_pool_contract_addresses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::DstPoolAddress,
                dst_pool_contract_addresses,
            ));
        }
    }
    if let Some(asset_symbols) = args.asset_symbol {
        if !asset_symbols.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::AssetSymbol,
                asset_symbols,
            ));
        }
    }
    if let Some(bridge_types) = args.bridge_type {
        if !bridge_types.is_empty() {
            let bridge_types = parse_bridge_types(&bridge_types);
            sub_filters.push(SubFilter::in_list(DepositColumn::BridgeType, bridge_types));
        }
    }
    if let Some(statuses) = args.status {
        if !statuses.is_empty() {
            let statuses = parse_deposit_statuses(&statuses);
            sub_filters.push(SubFilter::in_list(DepositColumn::Status, statuses));
        }
    }
    if let Some(commitment_hashes) = args.commitment_hash {
        if !commitment_hashes.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::CommitmentHash,
                commitment_hashes,
            ));
        }
    }
    if let Some(shielded_addresses) = args.shielded_address {
        if !shielded_addresses.is_empty() {
            sub_filters.push(SubFilter::in_list(
                DepositColumn::ShieldedRecipientAddress,
                shielded_addresses,
            ));
        }
    }
    let limit = args.limit.max(1_u64);
    let page = args.page.max(1_u64);
    let order_by = OrderBy::builder()
        .order(Order::Asc)
        .columns(vec![DocumentColumn::Id.to_string()])
        .build();
    let filter = QueryFilter::builder()
        .conditions(vec![Condition::from(sub_filters)])
        .conditions_operator(ConditionOperator::And)
        .limit(limit)
        .offset((page - 1) * limit)
        .order_by(order_by)
        .build();
    let deposits = mystiko.deposits.find(filter).await?;
    for deposit in deposits.into_iter() {
        print_deposit(deposit, pretty_json)?;
    }
    Ok(())
}

fn print_deposit(deposit: Deposit, pretty_json: bool) -> Result<(), MystikoCliError> {
    if pretty_json {
        println!("{}", serde_json::to_string_pretty(&deposit)?);
    } else {
        println!("{}", serde_json::to_string(&deposit)?);
    }
    Ok(())
}
