use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use mockall::mock;
use mystiko::MystikoCliArgs;
use mystiko_config::MystikoConfig;
use mystiko_core::{AccountHandler, Database, Mystiko, SynchronizerHandler, WalletHandler};
use mystiko_protos::core::synchronizer::v1::{ChainStatus, SyncOptions, SynchronizerStatus};
use mystiko_storage::SqlStatementFormatter;
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;

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
    let mystiko = setup(synchronizer).await;
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
        "--chain-ids",
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
    let mystiko = setup(synchronizer).await;
    let args =
        MystikoCliArgs::parse_from(["mystiko", "synchronizer", "status", "--with-contracts"]);
    mystiko::execute_with_mystiko(&mystiko, args.commands, false)
        .await
        .unwrap();
}

async fn setup(
    synchronizer: MockSynchronizer,
) -> Mystiko<SqlStatementFormatter, SqliteStorage, MockSynchronizer> {
    let database = Database::<SqlStatementFormatter, SqliteStorage>::new(
        SqlStatementFormatter::sqlite(),
        SqliteStorage::from_memory().await.unwrap(),
    );
    let database = Arc::new(database);
    let wallets = WalletHandler::new(database.clone());
    let accounts = AccountHandler::new(database.clone());
    let config = MystikoConfig::from_json_file("tests/files/config.json")
        .await
        .unwrap();
    Mystiko {
        db: database,
        config: Arc::new(config),
        accounts,
        wallets,
        synchronizer,
    }
}

mock! {
    #[derive(Debug)]
    Synchronizer {}

    #[async_trait]
    impl SynchronizerHandler<SyncOptions, SynchronizerStatus> for Synchronizer {
        type Error = anyhow::Error;
        async fn chain_synced_block(&self, chain_id: u64) -> Result<Option<u64>>;
        async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> Result<Option<u64>>;
        async fn status(&self, with_contracts: bool) -> Result<SynchronizerStatus>;
        async fn sync(&self, sync_option: SyncOptions) -> Result<()>;
    }
}
