use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use solana_program_pack::Pack;
use spl_token_interface;

use super::lib;
use crate::constants;
use crate::state::Pool;

pub fn add_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    fee: u16,
    amount_a: u64,
    amount_b: u64,
    pool_bump: u8,
    mint_pool_bump: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let payer = next_account_info(accounts_iter)?;
    let pool = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let pool_a = next_account_info(accounts_iter)?;
    let pool_b = next_account_info(accounts_iter)?;
    let mint_pool = next_account_info(accounts_iter)?;
    let payer_a = next_account_info(accounts_iter)?;
    let payer_b = next_account_info(accounts_iter)?;
    let payer_liq = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let ata_program = next_account_info(accounts_iter)?;
    let sys_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;

    // Verify payer is signer

    // Verify provided pool PDA matches the one calculated by lib::get_pool_pda

    // Verify provided mint_pool PDA matches the one calculated by lib::get_mint_pool_pda

    // Get Pool state

    // Verify Pool state mint_a = mint_a from accounts_iter

    // Verify Pool state mint_b = mint_b from accounts_iter

    // Get pool_a and pool_b amounts
    let pool_a_account = {
        let pool_a_data = pool_a.data.borrow();
        spl_token_interface::state::Account::unpack(&pool_a_data).unwrap()
    };
    let pool_a_amount = pool_a_account.amount;

    // Get mint_pool supply

    // Calculate shares to mint

    // Initialize payer_liq (associated token account for mint_pool owned by payer) if not initialized.

    // Transfer mint_a from payer to pool_a

    // Transfer mint_b from payer to pool_b

    // Mint LP tokens to payer

    Ok(())
}
