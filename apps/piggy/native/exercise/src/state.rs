use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Lock {
    // Destination to send SOL to
    pub dst: Pubkey,
    // Lock expiration timestamp
    pub exp: u64,
}
