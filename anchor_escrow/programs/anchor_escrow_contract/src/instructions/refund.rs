use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use crate::{error::EscrowError, state::Escrow};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut, 
        close = maker, 
        has_one = maker @ EscrowError::InvalidMaker, 
        has_one = mint_a @ EscrowError::InvalidMintA,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()], 
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut, 
        associated_token::mint = mint_a, 
        associated_token::authority = escrow, 
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a, 
        associated_token::authority = maker, 
        associated_token::token_program = token_program
    )]
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}

impl<'info> Refund<'info> {
    pub fn refund_tokens(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let maker_key = self.maker.key();
        let seed_ref = self.escrow.seed.to_le_bytes();
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            maker_key.as_ref(),
            seed_ref.as_ref(),
            &[self.escrow.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            &signer_seeds,
        );

        token_interface::transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        let cpi_accounts_2 = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let cpi_ctx_2 = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts_2,
            &signer_seeds,
        );

        token_interface::close_account(cpi_ctx_2)?;

        Ok(())
    }
}
