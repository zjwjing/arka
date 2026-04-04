//! Oracle module — price feeds and real-world data for agent decisions.

use crate::chain::Chain;

/// Oracle module for an agent.
pub struct OracleModule {
    chain: Chain,
}

impl OracleModule {
    pub fn new(chain: Chain) -> Self {
        Self { chain }
    }

    /// Get the chain this oracle operates on.
    pub fn chain(&self) -> Chain {
        self.chain
    }

    // TODO: price(pair) — get current price from Chainlink or TWAP
    // TODO: twap(pair, window) — time-weighted average price
    // TODO: historical(pair, periods) — historical price data
}
