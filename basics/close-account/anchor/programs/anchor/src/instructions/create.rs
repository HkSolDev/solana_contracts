use anchor_lang::prelude::*;
use crate::state::DataAccount;
use crate::constants::ANCHOR_DISCRIMINATOR;

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + DataAccount::INIT_SPACE,
        seeds = [b"create", user.key().as_ref()],
        bump
    )]
    pub data_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>,
}

pub fn process_create(ctx: Context<Create>, name: String) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    ctx.accounts.data_account.owner = ctx.accounts.user.key();
    ctx.accounts.data_account.bump = ctx.bumps.data_account;
    ctx.accounts.data_account.name = name;
    Ok(())
}
