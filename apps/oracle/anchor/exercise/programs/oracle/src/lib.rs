use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("4rx1qnz2jmfynAr8QrZcFFJdtrkRSiCBz56QrEE4eoqb");

#[program]
pub mod oracle {
    pub use super::instructions::*;
    use super::*;

    pub fn init(ctx: Context<Init>, price: u64) -> Result<()> {
        instructions::init(ctx, price)?;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, price: u64) -> Result<()> {
        // Write your code here
        instructions::update(ctx, price)?;
        Ok(())
    }
}
