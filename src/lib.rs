//! # arka
//!
//! Rust AI agent SDK for blockchain. Chain-agnostic wallets, DEX interaction,
//! MPP payments, on-chain state reading.

pub mod agent;
pub mod chain;
pub mod wallet;
pub mod tx;
pub mod dex;
pub mod mpp;
pub mod oracle;

mod error;
pub use error::{ArkaError, Result};

/// Convenience re-exports.
pub mod prelude {
    pub use crate::agent::{Agent, AgentBuilder};
    pub use crate::chain::Chain;
    pub use crate::wallet::Wallet;
    pub use crate::error::{ArkaError, Result};
    pub use alloy::primitives::{Address, U256};
}
