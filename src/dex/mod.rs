//! DEX interaction — swap, LP, route across decentralized exchanges.
//!
//! Supports Uniswap V3 (multi-chain), with Aerodrome/Velodrome and Trader Joe planned.

mod router;
mod types;

pub use router::UniswapV3Router;
pub use types::*;

use crate::chain::Chain;

/// DEX module for an agent — provides swap, quote, and LP operations.
pub struct DexModule {
    chain: Chain,
    router: UniswapV3Router,
}

impl DexModule {
    pub fn new(chain: Chain) -> Self {
        let router = UniswapV3Router::new(chain);
        Self { chain, router }
    }

    /// Get the chain this DEX module operates on.
    pub fn chain(&self) -> Chain {
        self.chain
    }

    /// Build a swap request.
    pub fn swap<'a>(
        &'a self,
        token_in: &'a str,
        token_out: &'a str,
        amount_in: alloy::primitives::U256,
    ) -> SwapBuilder<'a> {
        SwapBuilder {
            dex: self,
            token_in,
            token_out,
            amount_in,
            slippage_bps: 50, // 0.5% default
            fee_tier: FeeTier::Medium,
            deadline_secs: 300,
        }
    }

    /// Get the underlying router.
    pub fn router(&self) -> &UniswapV3Router {
        &self.router
    }
}

/// Builder pattern for constructing swap transactions.
pub struct SwapBuilder<'a> {
    dex: &'a DexModule,
    token_in: &'a str,
    token_out: &'a str,
    amount_in: alloy::primitives::U256,
    slippage_bps: u16,
    fee_tier: FeeTier,
    deadline_secs: u64,
}

impl<'a> SwapBuilder<'a> {
    /// Set slippage tolerance in basis points (e.g. 50 = 0.5%).
    pub fn slippage_bps(mut self, bps: u16) -> Self {
        self.slippage_bps = bps;
        self
    }

    /// Set the Uniswap V3 fee tier.
    pub fn fee_tier(mut self, tier: FeeTier) -> Self {
        self.fee_tier = tier;
        self
    }

    /// Set transaction deadline in seconds from now.
    pub fn deadline(mut self, secs: u64) -> Self {
        self.deadline_secs = secs;
        self
    }

    /// Build the swap parameters (does not execute).
    pub fn build(self) -> SwapParams {
        SwapParams {
            chain: self.dex.chain,
            token_in: self.token_in.to_string(),
            token_out: self.token_out.to_string(),
            amount_in: self.amount_in,
            slippage_bps: self.slippage_bps,
            fee_tier: self.fee_tier,
            deadline_secs: self.deadline_secs,
        }
    }
}
