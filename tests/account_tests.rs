use clap::Parser;
use mystiko::MystikoCliArgs;

#[allow(dead_code)]
mod common;

#[tokio::test]
async fn test_account_create() {
    let (_, db_path) = common::temp_db_path();
    create_wallet(&db_path).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "--pretty-json",
        "account",
        "create",
        "--password",
        "VeryAwes0meP@ssw0rd",
        "--name",
        "Test Account #1",
    ]);
    mystiko::execute(args).await.unwrap();
}

#[tokio::test]
async fn test_account_import() {
    let (_, db_path) = common::temp_db_path();
    create_wallet(&db_path).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "account",
        "import",
        "--password",
        "VeryAwes0meP@ssw0rd",
        "--name",
        "Test Account #1",
        "--secret-key",
        "a26dcc48d3731b3c2fcffae05d6b4999d864d02a74\
        59b25f6b8546b3d54b87c6e9941f4bbe39552ed83f0\
        d89e00ce4ebb69f75febf771dc5b554f3463710ab0a",
    ]);
    mystiko::execute(args).await.unwrap();
}

#[tokio::test]
async fn test_account_export_secret_key() {
    let (_, db_path) = common::temp_db_path();
    create_wallet(&db_path).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "account",
        "import",
        "--password",
        "VeryAwes0meP@ssw0rd",
        "--name",
        "Test Account #1",
        "--secret-key",
        "a26dcc48d3731b3c2fcffae05d6b4999d864d02a74\
        59b25f6b8546b3d54b87c6e9941f4bbe39552ed83f0\
        d89e00ce4ebb69f75febf771dc5b554f3463710ab0a",
    ]);
    mystiko::execute(args).await.unwrap();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "account",
        "export-secret-key",
        "--password",
        "VeryAwes0meP@ssw0rd",
        "--shielded-address",
        "7y5fEqG5ynEJYP6oPkriPJpBvFWvvGJzhReDSe9sHkmR1fWBWjm1qQLMNwpPNFnMFq3r9AudDMrS7CwetAx6ptzJH",
    ]);
    mystiko::execute(args).await.unwrap();
}

#[tokio::test]
async fn test_account_list() {
    let (_, db_path) = common::temp_db_path();
    create_wallet(&db_path).await;
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "account",
        "create",
        "--password",
        "VeryAwes0meP@ssw0rd",
        "--name",
        "Test Account #1",
    ]);
    mystiko::execute(args).await.unwrap();
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        &db_path,
        "account",
        "list",
    ]);
    mystiko::execute(args).await.unwrap();
}

async fn create_wallet(db_path: &str) {
    let args = MystikoCliArgs::parse_from([
        "mystiko",
        "--config-path",
        "tests/files/config.json",
        "--db-path",
        db_path,
        "wallet",
        "create",
        "--password",
        "VeryAwes0meP@ssw0rd",
    ]);
    mystiko::execute(args).await.unwrap();
}
