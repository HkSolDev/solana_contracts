use anchor_lang::prelude::*;
pub mod state;
pub mod instructions;

use crate::instructions::*;

declare_id!("9gsJBGRT1PfimyJt3CweNsnSoUyWTQdhsakYVDBLH7bV");

#[program]
pub mod anchor {
    use super::*;

    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        address: String,
        street: String,
        city: String,
        house_no: u8,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.create(name, address, street, city, house_no)?;
        Ok(())
    }
}
