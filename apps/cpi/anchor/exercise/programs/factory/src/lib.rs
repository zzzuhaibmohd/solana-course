use anchor_lang::prelude::*;

declare_id!("9WUK6RjaUJ2tY5ymQiKHtoXHX1B63F2XfcHbm6FShBGc");

// Automatically generate module using program idl found in ./idls
// Inport IDL for counter program

#[program]
pub mod factory {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        // Invoke the function init on the counter program
        Ok(())
    }

    pub fn inc(ctx: Context<Inc>) -> Result<()> {
        // Invoke the function inc on the counter program
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // Need to forward signature for account creation
    #[account(mut)]
    pub counter: Signer<'info>,
    // Add counter_program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Inc<'info> {
    /// CHECK: owned by the counter program
    #[account(mut)]
    pub counter: UncheckedAccount<'info>,
    // Add counter_program
}
