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

#[cfg(test)]
mod tests {
    use super::*;

    use tracing::Level;
    use tracing_subscriber::fmt::Subscriber;

    #[tokio::test]
    async fn test_start_test_view_server() {
        // Set up tracing subscriber with DEBUG level
        Subscriber::builder().with_max_level(Level::DEBUG).init();

        let _ = start_test_view_server().await.unwrap();

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
