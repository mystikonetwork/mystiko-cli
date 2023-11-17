#[allow(dead_code)]
mod common;

use crate::common::{mock_mystiko, MockSynchronizer};
use clap::Parser;
use mystiko::MystikoCliArgs;
use mystiko_protos::core::synchronizer::v1::{ChainStatus, SynchronizerStatus};

#[tokio::test]
async fn test_synchronizer_sync() {
    let mut synchronizer = MockSynchronizer::new();
    synchronizer
        .expect_sync()
        .withf(|options| {
            options.disable_datapacker_fetcher()
                && options.enable_datapacker_fetcher_validate()
                && options.disable_sequencer_fetcher()
                && options.enable_sequencer_fetcher_validate()
                && options.disable_provider_fetcher()
                && options.disable_provider_fetcher_validate()
                && options.disable_rule_validator()
                && options.disable_rule_validator_integrity_check()
                && options.disable_rule_validator_sequence_check()
                && options.disable_rule_validator_counter_check()
                && options.disable_rule_validator_tree_check()
                && options.fetcher_fetch_timeout_ms() == 3000_u64
                && options.fetcher_query_loaded_block_timeout_ms() == 5000_u64
                && options.validator_validate_concurrency() == 2
                && options.chain_ids == [56_u64]
        })
        .returning(|_| Ok(()));
    synchronizer
        .expect_status()
        .withf(|with_contracts| !(*with_contracts))
        .returning(|_| {
            Ok(SynchronizerStatus::builder()
                .chains(vec![ChainStatus::builder()
                    .chain_id(1_u64)
                    .synced_block(10000001_u64)
                    .build()])
                .build())
        });
    let mystiko = mock_mystiko(synchronizer).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "synchronizer",
        "sync",
        "--disable-datapacker-fetcher",
        "--enable-datapacker-fetcher-validate",
        "--disable-sequencer-fetcher",
        "--enable-sequencer-fetcher-validate",
        "--disable-provider-fetcher",
        "--disable-provider-fetcher-validate",
        "--disable-rule-validator",
        "--disable-rule-validator-integrity-check",
        "--disable-rule-validator-sequence-check",
        "--disable-rule-validator-counter-check",
        "--disable-rule-validator-tree-check",
        "--fetcher-fetch-timeout-ms",
        "3000",
        "--fetcher-query-loaded-block-timeout-ms",
        "5000",
        "--validator-validate-concurrency",
        "2",
        "--chain-id",
        "56",
    ]);
    mystiko::execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_synchronizer_status() {
    let mut synchronizer = MockSynchronizer::new();
    synchronizer
        .expect_status()
        .withf(|with_contracts| *with_contracts)
        .returning(|_| {
            Ok(SynchronizerStatus::builder()
                .chains(vec![ChainStatus::builder()
                    .chain_id(1_u64)
                    .synced_block(10000001_u64)
                    .build()])
                .build())
        });
    let mystiko = mock_mystiko(synchronizer).await;
    let args =
        MystikoCliArgs::parse_from(["mystiko", "synchronizer", "status", "--with-contracts"]);
    mystiko::execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_synchronizer_reset() {
    let mut synchronizer = MockSynchronizer::new();
    synchronizer
        .expect_reset()
        .withf(|options| {
            options.chains.len() == 1
                && options.chains[0].chain_id == 1
                && options.chains[0].contract_addresses == ["0x1234"]
                && options.chains[0].block_number == Some(10000001_u64)
        })
        .returning(|_| Ok(()));
    synchronizer
        .expect_status()
        .withf(|with_contracts| *with_contracts)
        .returning(|_| {
            Ok(SynchronizerStatus::builder()
                .chains(vec![ChainStatus::builder()
                    .chain_id(1_u64)
                    .synced_block(10000001_u64)
                    .build()])
                .build())
        });
    let mystiko = mock_mystiko(synchronizer).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "synchronizer",
        "reset",
        "--chain-id",
        "1",
        "--contract-address",
        "0x1234",
        "--to",
        "10000001",
    ]);
    mystiko::execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}
