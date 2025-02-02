use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
    TransferChecked,
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(address = listing.mint)]
    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [b"listing", marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl Delist<'_> {
    pub fn delist(&mut self) -> Result<()> {
        self.withdraw()?;
        self.close()?;
        Ok(())
    }

    fn withdraw(&mut self) -> Result<()> {
        let program = self.token_program.to_account_info();

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let seeds = &[
            b"listing",
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(program, accounts).with_signer(signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let seeds = &[
            b"listing",
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.listing.to_account_info(),
            },
        )
        .with_signer(signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
