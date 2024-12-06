use crate::common::{mock_mystiko, MockScanner};
use clap::Parser;
use mystiko::{execute_with_mystiko, MystikoCliArgs};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::scanner::v1::{AssetChainImportResult, AssetImportResult};

#[allow(dead_code)]
mod common;

#[tokio::test]
async fn test_scanner_scan() {
    let mut scanner = MockScanner::new();
    scanner
        .expect_scan()
        .withf(|options| {
            options.wallet_password == "test_password"
                && options.batch_size() == 100
                && options.concurrency() == 2
                && options.shielded_addresses == vec![String::from("test")]
        })
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(scanner).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "scanner",
        "scan",
        "--password",
        "test_password",
        "--batch-size",
        "100",
        "--concurrency",
        "2",
        "--shielded-address",
        "test",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_scanner_reset() {
    let mut scanner = MockScanner::new();
    scanner
        .expect_reset()
        .withf(|options| {
            options.reset_to_id() == "test_to_id"
                && options.shielded_addresses == vec![String::from("test")]
        })
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(scanner).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "scanner",
        "reset",
        "--to-id",
        "test_to_id",
        "--shielded-address",
        "test",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_scanner_import() {
    let mut scanner = MockScanner::new();
    scanner
        .expect_import()
        .withf(|options| options.chains[0].chain_id == 1)
        .returning(|_| {
            Ok(AssetImportResult::builder()
                .chains(vec![AssetChainImportResult::builder()
                    .chain_id(1u64)
                    .imported_count(1u32)
                    .found_count(1u32)
                    .build()])
                .build())
        });
    let mystiko = mock_mystiko(scanner).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "scanner",
        "import",
        "--password",
        "test_password",
        "--chain-id",
        "1",
        "--tx-hashes",
        "test",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_scanner_balance() {
    let mut scanner = MockScanner::new();
    scanner
        .expect_balance()
        .withf(|options| {
            options.with_spent()
                && options.chain_ids == vec![1_u64, 56_u64]
                && options.contract_addresses == vec!["test".to_string()]
                && options.shielded_addresses == vec!["test".to_string()]
                && options.asset_symbols == vec!["test".to_string()]
                && options.bridge_types == vec![BridgeType::Loop as i32]
        })
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(scanner).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "scanner",
        "balance",
        "--with-spent",
        "--chain-id",
        "1",
        "--chain-id",
        "56",
        "--contract-address",
        "test",
        "--shielded-address",
        "test",
        "--asset-symbol",
        "test",
        "--bridge-type",
        "loop",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_scanner_assets() {
    let mut scanner = MockScanner::new();
    scanner
        .expect_assets()
        .withf(|options| options.shielded_addresses == vec!["test".to_string()])
        .returning(|_| Ok(Default::default()));
    let mystiko = mock_mystiko(scanner).await;
    let args =
        MystikoCliArgs::parse_from(["mystiko", "scanner", "assets", "--shielded-address", "test"]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_scanner_chain_assets() {
    let mut scanner = MockScanner::new();
    scanner
        .expect_chain_assets()
        .withf(|chain_id, options| {
            *chain_id == 1_u64 && options.shielded_addresses == vec!["test".to_string()]
        })
        .returning(|_, _| Ok(Default::default()));
    let mystiko = mock_mystiko(scanner).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "scanner",
        "assets",
        "--chain-id",
        "1",
        "--shielded-address",
        "test",
    ]);
    execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}
