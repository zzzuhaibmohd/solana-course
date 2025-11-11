use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub mod instructions;
pub mod state;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Cmd {
    Lock {
        // Destination that will receive SOL after lock expiry
        dst: Pubkey,
        // Amount of SOL to lock
        amt: u64,
        // Lock expiration timestamp
        exp: u64,
        // PDA bump
        bump: u8,
    },
    Unlock {
        bump: u8,
    },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = Cmd::try_from_slice(instruction_data)?;

    match ix {
        Cmd::Lock {
            dst,
            amt,
            exp,
            bump,
        } => {
            instructions::lock(program_id, accounts, dst, amt, exp, bump)?;
        }
        Cmd::Unlock { bump } => {
            instructions::unlock(program_id, accounts, bump)?;
        }
    }

    Ok(())
}
