use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{error::EscrowError, state::Escrow};

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    /// Token Accounts
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,

    /// Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn taker(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            mint: self.mint_b.to_account_info(),
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        // The instruction expects the escrow.receive amount instead of the unvalidated amount variable
        token_interface::transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)?;

        let cpi_accounts_2 = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
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
        let cpi_ctx_2 = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts_2,
            &signer_seeds,
        );

        token_interface::transfer_checked(
            cpi_ctx_2,
            self.vault.amount,
            self.mint_a.decimals,
        )?;

        token_interface::close_account(CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.escrow.to_account_info(),
            },
            &signer_seeds,
        ))?;

        Ok(())
    }
}
