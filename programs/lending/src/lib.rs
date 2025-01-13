use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;
declare_id!("PuzF3FCpbFa3GKdaTAhw7kS5uojzrMzk2EwfsZsZ2pi");

#[program]
pub mod lending {
    use anchor_lang::solana_program::sysvar::instructions;

    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        process_init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount)
    }
}
