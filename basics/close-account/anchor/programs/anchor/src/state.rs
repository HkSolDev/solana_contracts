use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub owner: Pubkey,
    pub bump: u8,
    #[max_len(50)]
    pub name: String,
}