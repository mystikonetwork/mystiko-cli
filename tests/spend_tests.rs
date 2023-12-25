#[allow(dead_code)]
mod common;

use crate::common::{mock_mystiko, MockSpends};
use clap::Parser;
use mystiko::{execute_with_mystiko, MystikoCliArgs};
use mystiko_core::SpendColumn;
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::document::v1::Spend;
use mystiko_protos::core::v1::{SpendStatus, SpendType};
use mystiko_protos::storage::v1::{
    Condition, ConditionOperator, Order, OrderBy, QueryFilter, SubFilter,
};
use mystiko_storage::DocumentColumn;

#[tokio::test]
async fn test_spend_quote() {
    let mut spends = MockSpends::new();
    spends
        .expect_quote()
        .withf(|options| {
            options.chain_id == 5_u64
                && options.asset_symbol == "MTT"
                && options.bridge_type() == BridgeType::Tbridge
                && options.spend_type() == SpendType::Transfer
                && options.version() == 1_u32
                && options.query_timeout_ms() == 1000_u64
                && options.use_relayer()
                && options.amount() == 100_f64
        })
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(spends).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "spend",
        "quote",
        "--chain-id",
        "5",
        "--asset-symbol",
        "MTT",
        "--bridge-type",
        "tbridge",
        "--spend-type",
        "transfer",
        "--version",
        "1",
        "--query-timeout-ms",
        "1000",
        "--use-relayer",
        "--amount",
        "100",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_spend_create() {
    let mut spends = MockSpends::new();
    spends
        .expect_create()
        .withf(|options| {
            options.chain_id == 5_u64
                && options.asset_symbol == "MTT"
                && options.bridge_type() == BridgeType::Tbridge
                && options.spend_type() == SpendType::Transfer
                && options.version() == 1_u32
                && options.recipient == "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"
                && options.wallet_password == "password"
                && options.query_timeout_ms() == 400_u64
                && options.amount == 100_f64
                && options.rollup_fee_amount() == 1_f64
                && options.gas_relayer() == "relayer_01"
        })
        .returning(|_| {
            Ok(Spend {
                id: "1234".to_string(),
                ..Default::default()
            })
        });
    spends
        .expect_send()
        .withf(|options| {
            options.spend_id == "1234"
                && options.wallet_password == "password"
                && options.private_key() == "private_key"
                && options.tx_send_timeout_ms() == 1000_u64
                && options.tx_wait_timeout_ms() == 40000_u64
                && options.tx_wait_interval_ms() == 100_u64
                && options.spend_confirmations() == 6_u64
                && options.relayer_wait_timeout_ms() == 6000_u64
                && options.relayer_wait_interval_ms() == 200_u64
                && options.query_timeout_ms() == 400_u64
        })
        .returning(|_| {
            Ok(Spend {
                id: "1234".to_string(),
                ..Default::default()
            })
        });
    let mystiko = mock_mystiko(spends).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "spend",
        "create",
        "--chain-id",
        "5",
        "--asset-symbol",
        "MTT",
        "--bridge-type",
        "tbridge",
        "--spend-type",
        "transfer",
        "--version",
        "1",
        "--recipient",
        "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326",
        "--password",
        "password",
        "--amount",
        "100",
        "--rollup-fee",
        "1",
        "--relayer",
        "relayer_01",
        "--private-key",
        "private_key",
        "--tx-send-timeout-ms",
        "1000",
        "--tx-wait-timeout-ms",
        "40000",
        "--tx-wait-interval-ms",
        "100",
        "--confirmations",
        "6",
        "--relayer-wait-timeout-ms",
        "6000",
        "--relayer-wait-interval-ms",
        "200",
        "--query-timeout-ms",
        "400",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_spend_list() {
    let sub_filters = vec![
        SubFilter::in_list(SpendColumn::ChainId, vec![97_u64]),
        SubFilter::in_list(
            SpendColumn::ContractAddress,
            vec!["0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"],
        ),
        SubFilter::in_list(SpendColumn::AssetSymbol, vec!["MTT"]),
        SubFilter::in_list(SpendColumn::BridgeType, vec![BridgeType::Tbridge as i32]),
        SubFilter::in_list(SpendColumn::SpendType, vec![SpendType::Transfer as i32]),
        SubFilter::in_list(SpendColumn::Status, vec![SpendStatus::Succeeded as i32]),
        SubFilter::in_list(
            SpendColumn::Recipient,
            vec!["0xCBD6832Ebc203e49E2B771897067fce3c58575ac"],
        ),
        SubFilter::in_list(
            SpendColumn::GasRelayerAddress,
            vec!["0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD"],
        ),
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
    let mut spends = MockSpends::new();
    spends
        .expect_find::<QueryFilter>()
        .withf(move |filter| filter == &query_filter)
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(spends).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "spend",
        "list",
        "--chain-id",
        "97",
        "--contract-address",
        "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326",
        "--asset-symbol",
        "MTT",
        "--bridge-type",
        "tbridge",
        "--spend-type",
        "transfer",
        "--status",
        "succeeded",
        "--recipient",
        "0xCBD6832Ebc203e49E2B771897067fce3c58575ac",
        "--relayer",
        "0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD",
        "--limit",
        "20",
        "--page",
        "2",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}
