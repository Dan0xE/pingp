use std::net::ToSocketAddrs;

use anyhow::{Context, Result};
use is_url::is_url;
use log::{error, info};
use tokio::net::TcpStream;
use url::Url;

async fn connect(target: &str) -> Result<()> {
    info!("Connecting to {}", target);

    let response = match TcpStream::connect(target).await {
        Ok(response) => response,
        Err(e) => {
            error!("Host is either offline or does not exist: {}", e);
            return Err(e.into());
        }
    };

    info!(
        "Connected to {}",
        response
            .peer_addr()
            .context("Failed to get peer address")?
            .ip()
    );

    info!("Host Online");

    Ok(())
}

pub async fn ping(target: &str, port: Option<u16>) -> Result<()> {
    let target_addr = if is_url(target) {
        //We could also just try to parse the target and assume that on Url::parse failure that the provided target is an IP
        let url = Url::parse(target).context("Failed to parse URL")?;
        let host = url.host_str().context("URL does not contain a host")?;

        format!("{}:{}", host, port.unwrap_or(80))
    } else {
        format!("{}:{}", target, port.unwrap_or(80))
    };

    target_addr
        .to_socket_addrs()
        .context("Failed to convert into socket address")?
        .next()
        .context("Failed to convert into socket address")?;

    connect(target_addr.as_str())
        .await
        .context("Failed to connect to host")
}
