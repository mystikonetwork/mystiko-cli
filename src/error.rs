use thiserror::Error;

#[derive(Debug, Error)]
pub enum MystikoCliError {
    #[error(transparent)]
    MystikoError(#[from] mystiko_core::MystikoError),
    #[error(transparent)]
    AccountHandlerError(#[from] mystiko_core::AccountHandlerError),
    #[error(transparent)]
    WalletHandlerError(#[from] mystiko_core::WalletHandlerError),
    #[error(transparent)]
    SynchronizerError(#[from] mystiko_core::SynchronizerError),
    #[error(transparent)]
    ParseLevelError(#[from] log::ParseLevelError),
    #[error(transparent)]
    StorageError(#[from] mystiko_storage::StorageError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("mystiko_config raised error: {0}")]
    ConfigError(anyhow::Error),
}
