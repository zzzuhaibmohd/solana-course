use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program::{
    program::invoke_signed,
    system_instruction,
    sysvar::{Sysvar, rent::Rent},
};
use solana_program_pack::Pack;
use spl_token_interface;

use super::lib;
use crate::constants;
use crate::state::Pool;

pub fn init_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    pool_bump: u8,
    mint_pool_bump: u8,
) -> Result<(), ProgramError> {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let mint_pool = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let ata_program = next_account_info(accounts_iter)?;
    let sys_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    // Verify payer is signer

    // Check token decimals are equal

    // Verify pool, pool_a, pool_b and mint_pool accounts are not initialized

    // Verify provided pool PDA matches the one calculated by lib::get_pool_pda

    // Verify provided mint_pool PDA matches the one calculated by lib::get_mint_pool_pda

    // Create pool PDA
    let rent = Rent::get()?;

    // Create pool_a associated token account

    // Create pool_b associated token account

    // Create mint_pool PDA

    // Initialize mint_pool

    // Initialize pool state

    Ok(())
}
