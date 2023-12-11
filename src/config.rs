use anyhow::{Context, Result};
use log::warn;
use serde::Deserialize;
use std::convert::TryFrom;
use std::env;
use std::fs::File;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub log_level: Option<String>,
}

impl TryFrom<File> for Config {
    type Error = anyhow::Error;

    fn try_from(file: File) -> Result<Self> {
        let config = serde_yaml::from_reader(file).context("Failed to parse config file")?;
        Ok(config)
    }
}

impl TryFrom<&str> for Config {
    type Error = anyhow::Error;

    fn try_from(path: &str) -> Result<Self> {
        match File::open(path) {
            Ok(file) => Self::try_from(file),
            Err(e) => {
                warn!("Failed to open config file: {}, now reading env", e);
                let token = std::env::var("TOKEN").context("Failed to read TOKEN env")?;
                Ok(Config {
                    token,
                    log_level: Some(env::var("RUST_LOG").unwrap_or(String::from("warn"))),
                })
            }
        }
    }
}
