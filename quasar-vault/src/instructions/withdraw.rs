use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub user: &'info mut Signer,
    #[account(mut, seeds = [b"vault", user], bump)]
    pub vault: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

impl<'a> Withdraw<'a> {
    fn Withdraw(&self, amount: u64) -> Result<(), ProgramError> {
        let vault = self.vault.to_account_view();
        let user = self.user.to_account_view();

        set_lamports(vault, vault.lamports() - amount);
        set_lamports(user, user.lamports() - amount);
        Ok(())
    }
}
