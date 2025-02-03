use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, MintTo},
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    pub seller_token: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = seller_token,
        associated_token::authority = buyer,
    )]
    pub buyer_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = rewards,
        associated_token::authority = buyer,
    )]
    pub buyer_rewards_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [b"marketplace", marketplace.key().as_ref()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        mut,
        seeds = [b"listing", marketplace.key().as_ref(), seller_token.key().as_ref()], 
        bump = listing.bump,
        close = seller
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = seller_token,
        associated_token::authority = listing,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = rewards.decimals,
        mint::authority = marketplace,
    )]
    pub rewards: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl Purchase<'_> {
    pub fn purchase(&mut self) -> Result<()> {
        self.pay()?;
        self.send_nft()?;
        self.reward()?;
        self.close()?;
        Ok(())
    }

    fn pay(&mut self) -> Result<()> {
        let accounts_price = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        let accounts_fee = Transfer {
            from: self.seller.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx_fee = CpiContext::new(self.system_program.to_account_info(), accounts_fee);
        let fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee.checked_div(100).unwrap())
            .unwrap();

        let cpi_ctx_price = CpiContext::new(self.system_program.to_account_info(), accounts_price);
        let amount = self.listing.price.checked_sub(fee).unwrap();

        transfer(cpi_ctx_fee, fee)?;
        transfer(cpi_ctx_price, amount)?;

        Ok(())
    }

    fn send_nft(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.buyer_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.seller_token.to_account_info(),
        };

        let seeds = &[
            b"listing",
            &self.marketplace.key().to_bytes()[..],
            &self.seller_token.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts)
            .with_signer(signer_seeds);

        transfer_checked(cpi_ctx, 1, self.seller_token.decimals)?;

        Ok(())
    }

    fn reward(&mut self) -> Result<()> {
        let accounts = MintTo {
            mint: self.rewards.to_account_info(),
            to: self.buyer_rewards_ata.to_account_info(),
            authority: self.marketplace.to_account_info(),
        };

        let seeds = &[
            b"marketplace",
            &self.marketplace.key().to_bytes()[..],
            &[self.marketplace.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts)
            .with_signer(signer_seeds);

        mint_to(cpi_ctx, 1)?;

        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        let seeds = &[
            b"listing",
            &self.marketplace.key().to_bytes()[..],
            &self.seller_token.key().to_bytes()[..],
            &[self.listing.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.seller.to_account_info(),
                authority: self.listing.to_account_info(),
            },
        )
        .with_signer(signer_seeds);

        close_account(cpi_ctx)?;

        Ok(())
    }
}
