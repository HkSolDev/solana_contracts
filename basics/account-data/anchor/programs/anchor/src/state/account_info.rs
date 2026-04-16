use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account]
pub struct AddressInfo {
    #[max_len(50)]
    pub address: String,

    #[max_len(50)]
    pub name: String,
    #[max_len(50)]
    pub street: String,
    #[max_len(50)]
    pub city: String,
    pub owner: Pubkey,
    pub house_no: u8,
}
