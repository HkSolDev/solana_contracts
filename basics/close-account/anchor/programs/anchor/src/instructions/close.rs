use anchor_lang::prelude::*;
use crate::state::DataAccount;

#[derive(Accounts)]
pub struct Close<'a> {
    #[account(mut)]
    pub user: Signer<'a>,
    #[account(mut, close = user)]
    pub data_account: Account<'a, DataAccount>,
}

pub fn process_close(_ctx: Context<Close>) -> Result<()> {
    msg!("Closing account");
    Ok(())
}