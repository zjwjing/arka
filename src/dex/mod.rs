//! DEX interaction — swap, LP, route across decentralized exchanges.

use crate::chain::Chain;

/// DEX module for an agent.
pub struct DexModule {
    chain: Chain,
}

impl DexModule {
    pub fn new(chain: Chain) -> Self {
        Self { chain }
    }

    /// Get the chain this DEX module operates on.
    pub fn chain(&self) -> Chain {
        self.chain
    }

    // TODO: swap(), add_liquidity(), remove_liquidity(), get_quote()
    // These will be implemented per-DEX (Uniswap V3, Aerodrome, Trader Joe)
}
