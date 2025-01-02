use async_trait::async_trait;
use mockall::mock;
use mystiko_core::{
    Accounts, Database, DepositHandler, FromContext, Mystiko, MystikoContext, MystikoError,
    MystikoOptions, ScannerHandler, SpendHandler, SynchronizerHandler, TransactionSigner, Wallets,
};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::document::v1::{Deposit, Spend};
use mystiko_protos::core::handler::v1::{
    CreateDepositOptions, CreateSpendOptions, DepositQuote, DepositSummary,
    FixDepositStatusOptions, FixSpendStatusOptions, QuoteDepositOptions, QuoteSpendOptions,
    SendDepositOptions, SendSpendOptions, SpendQuote, SpendSummary,
};
use mystiko_protos::core::scanner::v1::{
    AssetImportOptions, AssetImportResult, AssetsByChain, AssetsOptions, BalanceOptions,
    BalanceResult, ScannerResetOptions, ScannerResetResult, ScannerScanOptions, ScannerScanResult,
    ScannerSyncOptions,
};
use mystiko_protos::core::synchronizer::v1::{
    SynchronizerResetOptions, SynchronizerStatus, SynchronizerSyncOptions,
};
use mystiko_protos::storage::v1::QueryFilter;
use mystiko_storage::{ColumnValues, SqlStatementFormatter, StatementFormatter, Storage};
use mystiko_storage_sqlite::SqliteStorage;
use std::sync::Arc;
use typed_builder::TypedBuilder;

pub fn temp_db_path() -> (tempfile::TempDir, String) {
    let db_folder = tempfile::tempdir().unwrap();
    let db_path = db_folder
        .path()
        .join("test.db")
        .to_string_lossy()
        .to_string();
    (db_folder, db_path)
}

pub async fn mock_mystiko<O>(mock_options: O) -> MockMystiko
where
    O: Into<MockMystikoOptions>,
{
    let mock_options = mock_options.into();
    let database = Database::<SqlStatementFormatter, SqliteStorage>::new(
        SqlStatementFormatter::sqlite(),
        SqliteStorage::from_memory().await.unwrap(),
    );
    let config_options = ConfigOptions::builder()
        .file_path("tests/files/config.json".to_string())
        .build();
    let options = MystikoOptions::builder()
        .config_options(config_options)
        .build();
    let context =
        MystikoContext::<SqlStatementFormatter, SqliteStorage>::new(database, Some(options))
            .await
            .unwrap();
    assert_eq!(context.config.version(), "0.1.11223344");
    MockMystiko {
        db: context.db.clone(),
        config: context.config.clone(),
        accounts: Accounts::<SqlStatementFormatter, SqliteStorage>::from_context(&context)
            .await
            .unwrap(),
        wallets: Wallets::<SqlStatementFormatter, SqliteStorage>::from_context(&context)
            .await
            .unwrap(),
        deposits: mock_options.deposits,
        spends: mock_options.spends,
        synchronizer: mock_options.synchronizer,
        scanner: mock_options.scanner,
    }
}

pub type MockMystiko = Mystiko<
    SqlStatementFormatter,
    SqliteStorage,
    Wallets<SqlStatementFormatter, SqliteStorage>,
    Accounts<SqlStatementFormatter, SqliteStorage>,
    MockDeposits,
    MockSpends,
    MockSynchronizer,
    MockScanner,
>;

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct MockMystikoOptions {
    pub deposits: MockDeposits,
    pub spends: MockSpends,
    pub scanner: MockScanner,
    pub synchronizer: MockSynchronizer,
}

mock! {
    #[derive(Debug, Default)]
    pub Deposits {}

    #[async_trait]
    impl DepositHandler<
        Deposit,
        QuoteDepositOptions,
        DepositQuote,
        CreateDepositOptions,
        DepositSummary,
        SendDepositOptions,
        FixDepositStatusOptions,
    > for Deposits {
        type Error = anyhow::Error;
        async fn quote(&self, options: QuoteDepositOptions) -> anyhow::Result<DepositQuote>;
        async fn summary(&self, options: CreateDepositOptions) ->  anyhow::Result<DepositSummary>;
        async fn create(&self, options: CreateDepositOptions) ->  anyhow::Result<Deposit>;
        async fn send(&self, options: SendDepositOptions) -> anyhow::Result<Deposit>;
        async fn send_with_signer<Signer>(&self, options: SendDepositOptions, signer: Arc<Signer>) -> anyhow::Result<Deposit>
        where
            Signer: TransactionSigner + 'static;
        async fn fix_status(&self, options: FixDepositStatusOptions) -> anyhow::Result<Deposit>;
        async fn find<Filter>(&self, filter: Filter) -> anyhow::Result<Vec<Deposit>>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn find_all(&self) -> anyhow::Result<Vec<Deposit>>;
        async fn find_one<Filter>(&self, filter: Filter) -> anyhow::Result<Option<Deposit>>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn find_by_id(&self, id: String) -> anyhow::Result<Option<Deposit>>;
        async fn count<Filter>(&self, filter: Filter) -> anyhow::Result<u64>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn count_all(&self) -> anyhow::Result<u64>;
        async fn update(&self, deposit: Deposit) -> anyhow::Result<Deposit>;
        async fn update_batch(&self, deposits: Vec<Deposit>) -> anyhow::Result<Vec<Deposit>>;
        async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> anyhow::Result<()>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static,
            Values: Into<ColumnValues> + Send + Sync + 'static;
        async fn update_all<Values>(&self, column_values: Values) -> anyhow::Result<()>
        where
            Values: Into<ColumnValues> + Send + Sync + 'static;
        async fn delete(&self, deposit: Deposit) -> anyhow::Result<()>;
        async fn delete_batch(&self, deposits: Vec<Deposit>) -> anyhow::Result<()>;
        async fn delete_by_filter<Filter>(&self, filter: Filter) -> anyhow::Result<()>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn delete_all(&self) -> anyhow::Result<()>;
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for MockDeposits
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(_context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(MockDeposits::new())
    }
}

impl From<MockDeposits> for MockMystikoOptions {
    fn from(value: MockDeposits) -> Self {
        MockMystikoOptions::builder().deposits(value).build()
    }
}

mock! {
    #[derive(Debug, Default)]
    pub Spends {}

    #[async_trait]
    impl SpendHandler<
        Spend,
        QuoteSpendOptions,
        SpendQuote,
        CreateSpendOptions,
        SpendSummary,
        SendSpendOptions,
        FixSpendStatusOptions,
    > for Spends {
        type Error = anyhow::Error;

        async fn quote(&self, options: QuoteSpendOptions) -> anyhow::Result<SpendQuote>;
        async fn summary(&self, options: CreateSpendOptions) -> anyhow::Result<SpendSummary>;
        async fn create(&self, options: CreateSpendOptions) -> anyhow::Result<Spend>;
        async fn send(&self, options: SendSpendOptions) -> anyhow::Result<Spend>;
        async fn send_with_signer<Signer>(&self, options: SendSpendOptions, signer: Arc<Signer>) -> anyhow::Result<Spend>
        where
            Signer: TransactionSigner + 'static;
        async fn fix_status(&self, options: FixSpendStatusOptions) -> anyhow::Result<Spend>;
        async fn find<Filter>(&self, filter: Filter) -> anyhow::Result<Vec<Spend>>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn find_all(&self) -> anyhow::Result<Vec<Spend>>;
        async fn find_one<Filter>(&self, filter: Filter) -> anyhow::Result<Option<Spend>>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn find_by_id(&self, id: String) -> anyhow::Result<Option<Spend>>;
        async fn count<Filter>(&self, filter: Filter) -> anyhow::Result<u64>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn count_all(&self) -> anyhow::Result<u64>;
        async fn update(&self, spend: Spend) -> anyhow::Result<Spend>;
        async fn update_batch(&self, spends: Vec<Spend>) -> anyhow::Result<Vec<Spend>>;
        async fn update_by_filter<Filter, Values>(&self, column_values: Values, filter: Filter) -> anyhow::Result<()>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static,
            Values: Into<ColumnValues> + Send + Sync + 'static;
        async fn update_all<Values>(&self, column_values: Values) -> anyhow::Result<()>
        where
            Values: Into<ColumnValues> + Send + Sync + 'static;
        async fn delete(&self, spend: Spend) -> anyhow::Result<()>;
        async fn delete_batch(&self, spends: Vec<Spend>) -> anyhow::Result<()>;
        async fn delete_by_filter<Filter>(&self, filter: Filter) -> anyhow::Result<()>
        where
            Filter: Into<QueryFilter> + Send + Sync + 'static;
        async fn delete_all(&self) -> anyhow::Result<()>;
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for MockSpends
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(_context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(MockSpends::new())
    }
}

impl From<MockSpends> for MockMystikoOptions {
    fn from(value: MockSpends) -> Self {
        MockMystikoOptions::builder().spends(value).build()
    }
}

mock! {
    #[derive(Debug, Default)]
    pub Scanner {}

    #[async_trait]
    impl ScannerHandler<
        ScannerSyncOptions,
        ScannerScanOptions,
        ScannerScanResult,
        ScannerResetOptions,
        ScannerResetResult,
        AssetImportOptions,
        AssetImportResult,
        BalanceOptions,
        BalanceResult,
        AssetsOptions,
        AssetsByChain,
    > for Scanner {
        type Error = anyhow::Error;
        async fn sync(&self, options: ScannerSyncOptions) -> anyhow::Result<BalanceResult>;
        async fn scan(&self, options: ScannerScanOptions) -> anyhow::Result<ScannerScanResult>;
        async fn reset(&self, options: ScannerResetOptions) -> anyhow::Result<ScannerResetResult>;
        async fn import(&self, options: AssetImportOptions) -> anyhow::Result<AssetImportResult>;
        async fn balance(&self, options: BalanceOptions) -> anyhow::Result<BalanceResult>;
        async fn assets(&self, options: AssetsOptions) -> anyhow::Result<Vec<AssetsByChain>>;
        async fn chain_assets(&self, chain_id: u64, options: AssetsOptions) -> anyhow::Result<Option<AssetsByChain>>;
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for MockScanner
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(_context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(MockScanner::new())
    }
}

impl From<MockScanner> for MockMystikoOptions {
    fn from(value: MockScanner) -> Self {
        MockMystikoOptions::builder().scanner(value).build()
    }
}

mock! {
    #[derive(Debug, Default)]
    pub Synchronizer {}

    #[async_trait]
    impl SynchronizerHandler<SynchronizerSyncOptions, SynchronizerStatus, SynchronizerResetOptions> for Synchronizer {
        type Error = anyhow::Error;
        async fn chain_synced_block(&self, chain_id: u64) -> anyhow::Result<Option<u64>>;
        async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> anyhow::Result<Option<u64>>;
        async fn status(&self, with_contracts: bool) -> anyhow::Result<SynchronizerStatus>;
        async fn sync(&self, sync_option: SynchronizerSyncOptions) -> anyhow::Result<SynchronizerStatus>;
        async fn reset(&self, reset_options: SynchronizerResetOptions) -> anyhow::Result<()>;
    }
}

#[async_trait]
impl<F, S> FromContext<F, S> for MockSynchronizer
where
    F: StatementFormatter,
    S: Storage,
{
    async fn from_context(_context: &MystikoContext<F, S>) -> Result<Self, MystikoError> {
        Ok(MockSynchronizer::new())
    }
}

impl From<MockSynchronizer> for MockMystikoOptions {
    fn from(value: MockSynchronizer) -> Self {
        MockMystikoOptions::builder().synchronizer(value).build()
    }
}
