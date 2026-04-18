pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Fc1HxCkd6Xynyvs1dt14sdLY3kJD5kFQtuoXXE7Fg6Bz");

#[program]
pub mod anchor {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String) -> Result<()> {
        process_create(ctx, name)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        process_close(ctx)
    }
}
