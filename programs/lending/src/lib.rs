use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;
declare_id!("PuzF3FCpbFa3GKdaTAhw7kS5uojzrMzk2EwfsZsZ2pi");

#[program]
pub mod lending {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        instructions::process_init_bank(ctx, liquidation_threshold, max_ltv)
    }
}
