use anchor_lang::prelude::*;

use crate::state::Oracle;

#[derive(Accounts)]
pub struct Update<'info> {
    // Write your code here
    pub owner: UncheckedAccount<'info>,
    // Write your code here
    #[account(mut, constraint = oracle.owner == oracle.key())]
    // Check oracle.owner == owner
    // oracle is deserialized to Oracle struct
    pub oracle: Account<'info, Oracle>,
}

pub fn update(ctx: Context<Update>, price: u64) -> Result<()> {
    // Write your code here
    Ok(())
}
