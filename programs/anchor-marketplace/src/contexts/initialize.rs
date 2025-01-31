use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{errors::MarketplaceError, state::Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_bytes()],
        bump,
        space = Marketplace::INIT_SPACE,
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn init(&mut self, name: String, fee: u64, bumps: &InitializeBumps) -> Result<()> {
        require!(
            !name.is_empty() && name.len() <= Marketplace::MAX_NAME_LEN,
            MarketplaceError::NameTooLong
        );

        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards,
            name,
        });

        Ok(())
    }
}
