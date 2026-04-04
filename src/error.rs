use thiserror::Error;

pub type Result<T> = std::result::Result<T, ArkaError>;

#[derive(Error, Debug)]
pub enum ArkaError {
    #[error("Chain error: {0}")]
    Chain(String),

    #[error("Wallet error: {0}")]
    Wallet(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("RPC error: {0}")]
    Rpc(String),

    #[error("DEX error: {0}")]
    Dex(String),

    #[error("MPP error: {0}")]
    Mpp(String),

    #[error("Oracle error: {0}")]
    Oracle(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance { have: String, need: String },

    #[error(transparent)]
    Alloy(#[from] alloy::contract::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
