use clap::Parser;
use mystiko::{execute, MystikoCliArgs};

#[tokio::test]
async fn test_about() {
    let args = MystikoCliArgs::parse_from(["mystiko", "about"]);
    execute(args).await.unwrap();
}
