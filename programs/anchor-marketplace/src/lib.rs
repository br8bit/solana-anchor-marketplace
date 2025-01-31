use anchor_lang::prelude::*;

declare_id!("DgjtkkbEJjHhdKG6QuCwCGHYaTvMFx8ahmuyby1yTfQh");

#[program]
pub mod sol_anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
