use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{Sysvar, clock::Clock},
};

use super::lib::get_pda;
use crate::state::Lock;

pub fn unlock(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bump: u8,
) -> Result<(), ProgramError> {
    let account_iter = &mut accounts.iter();
    let payer = next_account_info(account_iter)?;
    let pda = next_account_info(account_iter)?;
    let dst = next_account_info(account_iter)?;
    let sys_program = next_account_info(account_iter)?;

    // Check that the given account key matches expected PDA
    if *pda.key != get_pda(program_id, payer.key, dst.key, bump)? {
        return Err(ProgramError::InvalidSeeds);
    }

    // Load and validate lock state
    let (lock_dst, lock_exp) = {
        let data = pda.data.borrow();
        // TODO: fix?
        // let lock = Lock::try_from_slice(&data[..40])?;
        let lock = Lock::try_from_slice(&data)?;
        (lock.dst, lock.exp)
    }; // Drop borrow here

    // Verify destination matches
    if *dst.key != lock_dst {
        msg!("Destination mismatch");
        return Err(ProgramError::InvalidAccountData);
    }

    // Verify lock has expired
    let clock = Clock::get()?;
    let current_time: u64 = clock.unix_timestamp.try_into().unwrap();
    if lock_exp >= current_time {
        msg!("Lock not expired: exp={}, now={}", lock_exp, current_time);
        return Err(ProgramError::InvalidArgument);
    }

    // Get PDA balance and transfer lamports directly
    // Rent + locked amount
    let pda_lamports = pda.lamports();

    **pda.try_borrow_mut_lamports()? = 0;
    **dst.try_borrow_mut_lamports()? = dst
        .lamports()
        .checked_add(pda_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Clear out data
    pda.resize(0)?;

    // Assign the account to the System Program
    pda.assign(sys_program.key);

    Ok(())
}
