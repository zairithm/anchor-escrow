//! Escrow: a maker deposits token A and asks for an amount of token B.
//! A taker who sends the token B gets the token A, in one transaction.
//! Right now only `initialize` exists; the real instructions are in the issues.

// Anchor 0.31's `#[program]` macro calls `AccountInfo::realloc`, which newer
// solana crates mark deprecated. Remove this once Anchor is upgraded.
#![allow(deprecated)]

use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("anchor-escrow is deployed");
        Ok(())
    }

    // TODO(#1): make, take and refund.
}

#[derive(Accounts)]
pub struct Initialize {}

/// State stored for each open escrow.
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}
