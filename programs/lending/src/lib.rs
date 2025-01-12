use anchor_lang::prelude::*;

mod state;
declare_id!("PuzF3FCpbFa3GKdaTAhw7kS5uojzrMzk2EwfsZsZ2pi");

#[program]
pub mod lending {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
