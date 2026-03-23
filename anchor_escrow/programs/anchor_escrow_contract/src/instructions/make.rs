use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'a> {
    #[account(mut)]
    pub maker: Signer<'a>,
    #[account(
        init,
        payer = maker,
        space = Escrow::INIT_SPACE + Escrow::DISCRIMINATOR.len(),
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'a, Escrow>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'a, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'a, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'a, TokenAccount>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'a, TokenAccount>,

    pub associated_token_program: Program<'a, AssociatedToken>,
    pub token_program: Interface<'a, TokenInterface>,
    pub system_program: Program<'a, System>,
}

impl<'a> Make<'a> {
    pub fn initialize(&mut self, seed: u64, amount: u64, receive: u64, bump: u8) -> Result<()> {
        self.escrow.maker = self.maker.key();
        self.escrow.mint_a = self.mint_a.key();
        self.escrow.mint_b = self.mint_b.key();
        self.escrow.seed = seed;
        self.escrow.bump = bump;
        self.escrow.receive = receive;

        let cpi_acount = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.maker_ata_a.to_account_info(), // Updated name
            to: self.vault.to_account_info(),         // Updated name
            authority: self.maker.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_acount);

        token_interface::transfer_checked(cpi_context, amount, self.mint_a.decimals)?;

        Ok(())
    }
}
