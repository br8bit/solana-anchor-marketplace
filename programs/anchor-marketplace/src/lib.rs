use anchor_lang::prelude::*;

declare_id!("DgjtkkbEJjHhdKG6QuCwCGHYaTvMFx8ahmuyby1yTfQh");

mod contexts;
mod errors;
mod state;

use crate::contexts::*;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u64) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)?;
        Ok(())
    }
}
