use anchor_lang::prelude::*;

declare_id!("GaUggidmxsP3ZvfzbQ6AcxYmWimZerLqeoAGkHjhsiEk");

#[program]
pub mod switchboard_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
