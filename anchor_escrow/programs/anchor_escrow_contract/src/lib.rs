use anchor_lang::prelude::*;
mod instructions;
mod state;
use instructions::*;
mod error;
declare_id!("FiGeHoKfkSv2uTSYkyycrjri7a1PWzRpRyvEmeGGHDqG");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, amount: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
