use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub admin: Pubkey,
    pub fee: u64,
    pub bump: u8,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub name: String,
}

impl Marketplace {
    pub const MAX_NAME_LEN: usize = 32;
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 + 1 + 1 + (4 + Marketplace::MAX_NAME_LEN);
}
