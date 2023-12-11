use thiserror::Error;

#[derive(Error, Debug)]
pub enum MajimaError {
    #[error("configuration not found: {0}")]
    ConfigNotFound(#[from] anyhow::Error),
}
