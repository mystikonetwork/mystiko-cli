use async_trait::async_trait;
use mockall::mock;
use mystiko_core::{
    Accounts, Database, DepositHandler, FromContext, Mystiko, MystikoContext, MystikoError,
    MystikoOptions, ScannerHandler, SynchronizerHandler, TransactionSigner, Wallets,
};
use mystiko_protos::common::v1::ConfigOptions;
use mystiko_protos::core::document::v1::Deposit;
use mystiko_protos::core::handler::v1::{
    CreateDepositOptions, DepositQuote, DepositSummary, QuoteDepositOptions, SendDepositOptions,
};
use mystiko_protos::core::scanner::v1::{
    BalanceOptions, BalanceResult, ResetOptions, ResetResult, ScanOptions, ScanResult,
};
use mystiko_protos::core::synchronizer::v1::{SyncOptions, SynchronizerStatus};
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
    MockSynchronizer,
    MockScanner,
>;

#[derive(Debug, Default, TypedBuilder)]
#[builder(field_defaults(default, setter(into)))]
pub struct MockMystikoOptions {
    pub deposits: MockDeposits,
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
    > for Deposits {
        type Error = anyhow::Error;
        async fn quote(&self, options: QuoteDepositOptions) -> anyhow::Result<DepositQuote>;
        async fn summary(&self, options: CreateDepositOptions) ->  anyhow::Result<DepositSummary>;
        async fn create(&self, options: CreateDepositOptions) ->  anyhow::Result<Deposit>;
        async fn send(&self, options: SendDepositOptions) -> anyhow::Result<Deposit>;
        async fn send_with_signer<Signer>(&self, options: SendDepositOptions, signer: Arc<Signer>) -> anyhow::Result<Deposit>
        where
            Signer: TransactionSigner + 'static;
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
    pub Scanner {}

    #[async_trait]
    impl ScannerHandler<
        ScanOptions,
        ScanResult,
        ResetOptions,
        ResetResult,
        BalanceOptions,
        BalanceResult,
    > for Scanner {
        type Error = anyhow::Error;
        async fn scan(&self, options: ScanOptions) -> anyhow::Result<ScanResult>;
        async fn reset(&self, options: ResetOptions) -> anyhow::Result<ResetResult>;
        async fn balance(&self, options: BalanceOptions) -> anyhow::Result<BalanceResult>;
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
    impl SynchronizerHandler<SyncOptions, SynchronizerStatus> for Synchronizer {
        type Error = anyhow::Error;
        async fn chain_synced_block(&self, chain_id: u64) -> anyhow::Result<Option<u64>>;
        async fn contract_synced_block(&self, chain_id: u64, contract_address: &str) -> anyhow::Result<Option<u64>>;
        async fn status(&self, with_contracts: bool) -> anyhow::Result<SynchronizerStatus>;
        async fn sync(&self, sync_option: SyncOptions) -> anyhow::Result<()>;
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
