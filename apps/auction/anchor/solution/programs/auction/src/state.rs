use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Auction {
    pub sell_mint: Pubkey,
    pub buy_mint: Pubkey,
    // Price of 1 sell token, 1e6 = 1 buy token
    pub start_price: u64,
    pub end_price: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub seller: Pubkey,
}

impl Auction {
    pub const SEED_PREFIX: &'static [u8; 7] = b"auction";
}
