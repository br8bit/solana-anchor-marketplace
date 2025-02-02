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

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create(price, &ctx.bumps)?;
        ctx.accounts.deposit()?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()?;
        Ok(())
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase()?;
        Ok(())
    }
}
