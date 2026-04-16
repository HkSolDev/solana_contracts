use anchor_lang::prelude::*;
use crate::state::AddressInfo;
#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + AddressInfo::INIT_SPACE,
    )]
    pub address_info: Account<'info, AddressInfo>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateAddressInfo<'info> {
    pub fn create(&mut self, name: String, address: String, street: String, city: String, house_no: u8) -> Result<()> {
        self.address_info.name = name;
        self.address_info.address = address;
        self.address_info.street = street;
        self.address_info.city = city;
        self.address_info.owner = self.user.key();
        self.address_info.house_no = house_no;
        Ok(())
    }
}