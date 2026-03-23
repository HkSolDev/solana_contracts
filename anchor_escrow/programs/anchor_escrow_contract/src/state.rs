use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[derive(InitSpace)]
#[account(discriminator = 1)]
//This struct will tell what the on chain account data will be store its
//define the shape of the data
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}
