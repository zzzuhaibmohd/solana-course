use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, CloseAccount, Mint, TokenAccount, TokenInterface,
    },
};

use super::lib;
use crate::error;
use crate::state;

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: Receiver of PDA rents
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,

    pub mint_sell: InterfaceAccount<'info, Mint>,
    pub mint_buy: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [
            state::Auction::SEED_PREFIX,
            seller.key().as_ref(),
            mint_sell.key().as_ref(),
            mint_buy.key().as_ref()
        ],
        bump,
        close = seller,
    )]
    pub auction: Account<'info, state::Auction>,

    #[account(
        mut,
        associated_token::mint = mint_sell,
        associated_token::authority = auction,
    )]
    pub auction_sell_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_buy,
        associated_token::authority = buyer,
    )]
    pub buyer_buy_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_sell,
        associated_token::authority = buyer,
    )]
    pub buyer_sell_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_buy,
        associated_token::authority = seller,
    )]
    pub seller_buy_ata: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy(ctx: Context<Buy>, max_price: u64) -> Result<()> {
    let clock = Clock::get()?;
    let now = u64::try_from(clock.unix_timestamp).unwrap();

    // Check auction has started

    // Check auction has not ended

    // Calculate price

    // Check current price is greater than or equal to end_price

    // Check current price is less than or equal to max_price

    // Calculate amount of buy token to send to seller

    // Send buy token to seller

    // Send sell token to buyer
    let seeds: &[&[u8]] = &[
        state::Auction::SEED_PREFIX,
        &ctx.accounts.seller.key().to_bytes(),
        &ctx.accounts.mint_sell.key().to_bytes(),
        &ctx.accounts.mint_buy.key().to_bytes(),
        &[ctx.bumps.auction],
    ];

    // Close auction_sell_ata

    Ok(())
}
