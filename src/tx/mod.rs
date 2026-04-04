//! Transaction building, gas estimation, and simulation.

use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::chain::Chain;

/// A transaction request before signing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxRequest {
    pub chain: Chain,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub gas_limit: Option<u64>,
    pub max_fee_per_gas: Option<u128>,
    pub max_priority_fee: Option<u128>,
}

impl TxRequest {
    pub fn new(chain: Chain, to: Address) -> Self {
        Self {
            chain,
            to,
            value: U256::ZERO,
            data: Vec::new(),
            gas_limit: None,
            max_fee_per_gas: None,
            max_priority_fee: None,
        }
    }

    pub fn value(mut self, value: U256) -> Self {
        self.value = value;
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }

    pub fn gas_limit(mut self, limit: u64) -> Self {
        self.gas_limit = Some(limit);
        self
    }
}

/// Result of a submitted transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxResult {
    pub hash: String,
    pub chain: Chain,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
    pub success: bool,
}
