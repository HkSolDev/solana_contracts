use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[derive(InitSpace)]
#[account(discriminator = 1)]
//This struct will tell what the on chain account data will be store its
//define the shape of the data
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64, // How much amount the maker wants of token_b
    pub bump: u8,     //save the bump for the so can save the compute
    pub seed: u64,    // this is the random no which help to open the multiple
                      // Escrow with the same token
}
