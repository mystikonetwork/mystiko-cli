#[allow(dead_code)]
mod common;

use crate::common::{mock_mystiko, MockDeposits};
use clap::Parser;
use mystiko::{execute_with_mystiko, MystikoCliArgs};
use mystiko_core::DepositColumn;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Deposit;
use mystiko_protos::core::v1::DepositStatus;
use mystiko_protos::storage::v1::{
    Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter,
};
use mystiko_storage::DocumentColumn;

#[tokio::test]
async fn test_deposit_quote() {
    let mut deposits = MockDeposits::new();
    deposits
        .expect_quote()
        .withf(|options| {
            options.chain_id == 97_u64
                && options.asset_symbol == "MTT"
                && options.bridge_type() == BridgeType::Tbridge
                && options.dst_chain_id() == 5_u64
                && options.query_timeout_ms() == 1000_u64
        })
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(deposits).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "deposit",
        "quote",
        "--chain-id",
        "97",
        "--asset-symbol",
        "MTT",
        "--bridge-type",
        "tbridge",
        "--dst-chain-id",
        "5",
        "--query-timeout-ms",
        "1000",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_deposit_create() {
    let mut deposits = MockDeposits::new();
    deposits
        .expect_create()
        .withf(|options| {
            options.chain_id == 97_u64
                && options.asset_symbol == "MTT"
                && options.bridge_type() == BridgeType::Tbridge
                && options.dst_chain_id() == 5_u64
                && options.shielded_address == "Shielded_Address"
                && options.amount == 123.0_f64
                && options.rollup_fee_amount() == 0.01_f64
                && options.bridge_fee_amount() == 0.02_f64
                && options.executor_fee_amount() == 0.03_f64
                && options.query_timeout_ms() == 1000_u64
        })
        .returning(|_| {
            Ok(Deposit {
                id: "1234".to_string(),
                ..Default::default()
            })
        });
    deposits
        .expect_send()
        .withf(|options| {
            options.deposit_id == "1234"
                && options.private_key() == "private_key"
                && options.query_timeout_ms() == 1000_u64
                && options.asset_approve_confirmations() == 1_u64
                && options.deposit_confirmations() == 1_u64
                && options.tx_wait_interval_ms() == 1000_u64
                && options.tx_wait_timeout_ms() == 1000_u64
                && options.tx_send_timeout_ms() == 1000_u64
        })
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(deposits).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "deposit",
        "create",
        "--chain-id",
        "97",
        "--asset-symbol",
        "MTT",
        "--bridge-type",
        "tbridge",
        "--dst-chain-id",
        "5",
        "--shielded-address",
        "Shielded_Address",
        "--amount",
        "123",
        "--rollup-fee",
        "0.01",
        "--bridge-fee",
        "0.02",
        "--executor-fee",
        "0.03",
        "--private-key",
        "private_key",
        "--query-timeout-ms",
        "1000",
        "--asset-approve-confirmations",
        "1",
        "--deposit-confirmations",
        "1",
        "--tx-wait-interval-ms",
        "1000",
        "--tx-wait-timeout-ms",
        "1000",
        "--tx-send-timeout-ms",
        "1000",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_deposit_list() {
    let sub_filters = vec![
        SubFilter::in_list(DepositColumn::ChainId, vec![97_u64]),
        SubFilter::in_list(DepositColumn::ContractAddress, vec![String::from("0x1234")]),
        SubFilter::in_list(DepositColumn::PoolAddress, vec![String::from("0x5678")]),
        SubFilter::in_list(DepositColumn::DstChainId, vec![5_u64]),
        SubFilter::in_list(
            DepositColumn::DstChainContractAddress,
            vec![String::from("0x9012")],
        ),
        SubFilter::in_list(DepositColumn::DstPoolAddress, vec![String::from("0x3456")]),
        SubFilter::in_list(DepositColumn::AssetSymbol, vec![String::from("MTT")]),
        SubFilter::in_list(DepositColumn::BridgeType, vec![BridgeType::Tbridge as i32]),
        SubFilter::in_list(DepositColumn::Status, vec![DepositStatus::Queued as i32]),
        SubFilter::in_list(DepositColumn::CommitmentHash, vec![String::from("0x7890")]),
        SubFilter::in_list(DepositColumn::ShieldedAddress, vec![String::from("0x1234")]),
    ];
    let order_by = OrderBy::builder()
        .order(Order::Desc)
        .columns(vec![DocumentColumn::Id.to_string()])
        .build();
    let query_filter = QueryFilter::builder()
        .conditions(vec![Condition::from(sub_filters)])
        .conditions_operator(ConditionOperator::And)
        .order_by(order_by)
        .limit(20_u64)
        .offset(20_u64)
        .build();
    let mut deposits = MockDeposits::new();
    deposits
        .expect_find::<QueryFilter>()
        .withf(move |filter| filter == &query_filter)
        .returning(|_| Ok(vec![Default::default()]));
    let mystiko = mock_mystiko(deposits).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "deposit",
        "list",
        "--chain-id",
        "97",
        "--deposit-contract-address",
        "0x1234",
        "--pool-contract-address",
        "0x5678",
        "--dst-chain-id",
        "5",
        "--dst-deposit-contract-address",
        "0x9012",
        "--dst-pool-contract-address",
        "0x3456",
        "--asset-symbol",
        "MTT",
        "--bridge-type",
        "tbridge",
        "--status",
        "queued",
        "--commitment-hash",
        "0x7890",
        "--shielded-address",
        "0x1234",
        "--limit",
        "20",
        "--page",
        "2",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}
