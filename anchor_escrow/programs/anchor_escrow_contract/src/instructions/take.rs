use anchor_lang::prelude::*;
// NEW MISTAKE: You missed importing `CloseAccount` to close the vault account later.
// IMPROVEMENT: Add `CloseAccount` to the `token_interface` imports.
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked, CloseAccount},
};

use crate::state::Escrow;

#[derive(Accounts)]
// MISTAKE: You shouldn't need a seed instruction parameter here since `Take` only closes the escrow and reads the seed from `escrow` account's stored data.
// IMPROVEMENT: Remove `#[instruction(seed:u64)]`.
#[instruction(seed:u64)]
pub struct Take<'a> {
    #[account(mut)]
    pub taker: Signer<'a>,
    #[account(mut)]
    pub maker: SystemAccount<'a>,

    // SOLVED: Replaced `init` with `mut, close=maker` and updated seeds.
    // STILL MISSING: You didn't add the `has_one` constraints (e.g., `has_one = maker`, `has_one = mint_a`, `has_one = mint_b`). These are crucial for security to prevent a taker from passing in a fake maker or fake mints.
    // IMPROVEMENT: Add `has_one = maker, has_one = mint_a, has_one = mint_b` inside `#[account(...)]`.
    #[account(mut,close=maker, seeds=[b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()], bump)]
    pub escrow: Account<'a, Escrow>,

    pub mint_b: InterfaceAccount<'a, Mint>,

    // PARTIALLY SOLVED: You fixed the typo for authority.
    // STILL MISSING: You need `= token_program` for `associated_token::token_program`. Without it, Anchor uses the default spl-token instead of supporting Token2022 if applicable.
    // IMPROVEMENT: Change to `associated_token::token_program = token_program`.
    #[account(mut, associated_token::mint=mint_b, associated_token::authority = taker, associated_token::token_program)]
    pub taker_mint_b_ata: InterfaceAccount<'a, TokenAccount>,

    pub mint_a: InterfaceAccount<'a, Mint>,

    // PARTIALLY SOLVED: You updated the mint to `mint_b`.
    // STILL MISSING: 1. It still says `associated::authority`, it needs to be `associated_token::authority`.
    // STILL MISSING: 2. Missing `= token_program` for `associated_token::token_program`.
    // STILL MISSING: 3. If the maker doesn't have a `mint_b` account yet, this fails. It should use `init_if_needed`, `payer = taker`.
    // IMPROVEMENT: `#[account(init_if_needed, payer = taker, associated_token::mint = mint_b, associated_token::authority = maker, associated_token::token_program = token_program)]`
    #[account(mut, associated_token::mint=mint_b, associated::authority=maker, associated_token::token_program)]
    pub maker_mint_b_ata: InterfaceAccount<'a, TokenAccount>,

    // NEW MISTAKE: You created `taker_mint_a_ata` but set `mint=mint_b` instead of `mint=mint_a`.
    // NEW MISTAKE: Typo `associated::authority` instead of `associated_token::authority`.
    // NEW MISTAKE: Missing `= token_program` for `associated_token::token_program`.
    // NEW MISTAKE: Should use `init_if_needed, payer = taker` in case the taker doesn't have a mint_a account yet.
    // IMPROVEMENT: `#[account(init_if_needed, payer=taker, associated_token::mint=mint_a, associated_token::authority=taker, associated_token::token_program=token_program)]`
    #[account(mut, associated_token::mint=mint_b, associated::authority=taker, associated_token::token_program)]
    pub taker_mint_a_ata: InterfaceAccount<'a, TokenAccount>,
    
    // SOLVED: You correctly changed the vault to `mut` and linked it to `mint_a`.
    // The valut to hold the token mint_b
    #[account(mut,associated_token::mint=mint_a, associated_token::token_program = token_program, associated_token::authority=escrow)]
    pub mint_a_vault: InterfaceAccount<'a, TokenAccount>,
    pub token_program: Interface<'a, TokenInterface>,
    pub associated_token_program: Program<'a, AssociatedToken>,
    pub system_program: Program<'a, System>,
}

impl<'a> Take<'a> {
    pub fn taker(&mut self, seed: u64, amount: u64, bump: u8) -> Result<()> {
        // SOLVED: Successfully changed the destination to `maker_mint_b_ata` instead of vault.
        let cpi_accounts = TransferChecked {
            mint: self.mint_b.to_account_info(),
            from: self.taker_mint_b_ata.to_account_info(),
            to: self.maker_mint_b_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        // MISTAKE: The transfer to maker should receive `self.escrow.receive` amount instead of the unvalidated `amount` argument.
        // IMPROVEMENT: `token_interface::transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)?`
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token_interface::transfer_checked(cpi_ctx, amount, self.mint_b.decimals)?;

        let cpi_accounts_2 = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.mint_a_vault.to_account_info(),
            to: self.taker_mint_a_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        // NEW MISTAKE (THE RUST ERROR): By splitting the array binding `let seed = [...]` out, intermediate values like `&[self.escrow.bump]` die before `token_interface::transfer_checked`!
        // This is why you get "temporary value dropped while borrowed".
        // IMPROVEMENT: Combine the declaration to automatically extend compiler lifetimes, using the `signer_seeds: [&[&[u8]]; 1]` syntax!
        // Like this:
        // let signer_seeds: [&[&[u8]]; 1] = [&[ b"escrow", self.maker.to_account_info().key.as_ref(), &self.escrow.seed.to_le_bytes()[..], &[self.escrow.bump] ]];
        let maker_key = self.maker.key();
        let seed_ref = seed.to_le_bytes();
        let seed = [
            b"escrow",
            maker_key.as_ref(),
            seed_ref.as_ref(),
            &[self.escrow.bump],
        ];
        let cpi_ctx_2 = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts_2,
            &[&seed],
        );
        // MISTAKE: Draining the vault should use `self.mint_a_vault.amount`, not the `amount` input variable.
        // IMPROVEMENT: `token_interface::transfer_checked(cpi_ctx_2, self.mint_a_vault.amount, self.mint_a.decimals)?`
        token_interface::transfer_checked(cpi_ctx_2, amount, self.mint_a.decimals)?;

        // STILL MISSING: You transferred the tokens out of the vault, but you never closed the empty vault!
        // IMPROVEMENT: Add a CPI call here to `token_interface::close_account(...)` targeting `self.mint_a_vault` and granting the remaining compute/rent lamports to `self.maker`. 
        // Use `CpiContext::new_with_signer` in the same way you did for the transfer.

        Ok(())
    }
}
