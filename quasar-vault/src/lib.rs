#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;

mod errors;
mod instructions;
mod state;
use instructions::*;

declare_id!("4jFLNEnnhszXpsYyRkx44gZSjYU5S7DykXe5vYog8Kkc");

#[program]
mod quasar_vault {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn deposit(ctx: Ctx<Deposit>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.deposit(amount)
    }
}

#[cfg(test)]
mod tests;
