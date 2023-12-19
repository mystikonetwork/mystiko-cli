#[allow(dead_code)]
mod common;

use clap::Parser;
use mystiko::MystikoCliArgs;

#[tokio::test]
async fn test_wallet_create() {
    let (_, db_path) = common::temp_db_path();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "--compact-json",
        "wallet",
        "create",
        "--password",
        "VeryAwes0meP@ssw0rd",
    ]);
    mystiko::execute(args).await.unwrap();
}

#[tokio::test]
async fn test_wallet_import() {
    let (_, db_path) = common::temp_db_path();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "wallet",
        "import",
        "--password",
        "VeryAwes0meP@ssw0rd",
        "--mnemonic",
        "duty hawk source husband \
        cabin pencil airport crawl \
        denial urban cable know \
        obtain giant mirror taste \
        fossil ethics circle behave \
        hedgehog supreme artefact situate",
    ]);
    mystiko::execute(args).await.unwrap();
}

#[tokio::test]
async fn test_wallet_export_mnemonic_phrase() {
    let (_, db_path) = common::temp_db_path();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "wallet",
        "create",
        "--password",
        "VeryAwes0meP@ssw0rd",
    ]);
    mystiko::execute(args).await.unwrap();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "wallet",
        "export-mnemonic",
        "--password",
        "VeryAwes0meP@ssw0rd",
    ]);
    mystiko::execute(args).await.unwrap();
}

#[tokio::test]
async fn test_wallet_update_password() {
    let (_, db_path) = common::temp_db_path();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "wallet",
        "create",
        "--password",
        "VeryAwes0meP@ssw0rd",
    ]);
    mystiko::execute(args).await.unwrap();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "wallet",
        "update-password",
        "--old",
        "VeryAwes0meP@ssw0rd",
        "--new",
        "VeryAwes0meP@ssw0rd2",
    ]);
    mystiko::execute(args).await.unwrap();
}
