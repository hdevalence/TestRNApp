use anyhow::Result;
use penumbra_view::{Storage, ViewServer};

const ENDPOINT: &str = "https://testnet.plinfra.net";

pub async fn start_test_view_server() -> Result<ViewServer> {
    ViewServer::load_or_initialize(
        None::<&str>,
        None::<&str>,
        &penumbra_keys::test_keys::FULL_VIEWING_KEY,
        ENDPOINT.parse().unwrap(),
    )
    .await
}
