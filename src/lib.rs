use anyhow::{Context, Result};
use utils::ping::ping;

mod utils;

pub async fn perform_ping(url: &str, port: Option<u16>) -> Result<(), String> {
    ping(url, port)
        .await
        .context("Failed to connect to host")
        .map_err(|e| {
            e.chain()
                .map(|cause| cause.to_string())
                .collect::<Vec<_>>()
                .join(": ")
        })
}
