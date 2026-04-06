use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        require_gt!(
            amount,
            Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        let inx = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let program = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(program, inx);
        transfer(cpi_context, amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        
        let bump = ctx.bumps.vault;
        let amount = ctx.accounts.vault.lamports(); 

        let inx = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        };

        let signer_seeds = &[
            b"vault",
            ctx.accounts.signer.to_account_info().key.as_ref(),
            &[bump],
        ];
        let signer_seeds_outer = &[&signer_seeds[..]];

        let program = ctx.accounts.system_program.to_account_info();
        let cpi_context = CpiContext::new(program, inx).with_signer(signer_seeds_outer);

        transfer(cpi_context, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}
