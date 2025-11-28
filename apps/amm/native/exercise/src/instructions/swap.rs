use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use super::lib;
use crate::constants;
use crate::state::Pool;

pub fn swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    a_for_b: bool,
    amount_in: u64,
    min_amount_out: u64,
    pool_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let payer_a = next_account_info(accounts_iter)?;
    let payer_b = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Verify payer is signer

    // Verify provided pool PDA matches the one calculated by lib::get_pool_pda

    // Get Pool state

    // Verify Pool state mint_a = mint_a from accounts_iter

    // Verify Pool state mint_b = mint_b from accounts_iter

    // Calculate amount out with fee

    // Check amount out is >= minimum amount specified by payer

    // Determine swap direction

    // Transfer token from payer to pool

    // Transfer token from pool to payer

    Ok(())
}
