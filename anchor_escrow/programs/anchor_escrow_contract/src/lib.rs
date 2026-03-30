use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;
mod error;
declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;
    #[instruction(discriminator = 0)]
    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        ctx.accounts.initialize(seed, amount, receive, ctx.bumps.escrow)
    }
    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.taker()
    }
    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_tokens()
    }
}

