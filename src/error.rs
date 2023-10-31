use thiserror::Error;

#[derive(Debug, Error)]
pub enum MystikoCliError {
    #[error(transparent)]
    MystikoError(#[from] mystiko_core::MystikoError),
    #[error(transparent)]
    ParseLevelError(#[from] log::ParseLevelError),
    #[error(transparent)]
    StorageError(#[from] mystiko_storage::StorageError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("mystiko_config raised error: {0}")]
    ConfigError(anyhow::Error),
}
