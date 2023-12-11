use anyhow::{Context, Result};
use fxtd::{bot::Handler, config::Config, error::MajimaError};
use log::{error, info};
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::try_from("token.yml").map_err(|e| MajimaError::from(e))?;
    env::set_var(
        "RUST_LOG",
        config
            .log_level
            .unwrap_or(env::var("RUST_LOG").unwrap_or(String::from("warn"))),
    );
    env_logger::init();
    info!("Starting FXT Discord Bot");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(config.token, intents)
        .event_handler(Handler)
        .await
        .with_context(|| {
            error!("Failed to create client");
            "Failed to create client"
        })?;

    client.start().await.with_context(|| {
        error!("Failed to start client");
        "Failed to start client"
    })
}
