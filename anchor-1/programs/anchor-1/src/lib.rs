use anchor_lang::prelude::*;

declare_id!("Uai7p1eu51tjTYsEmRtWfr2A4QgeGp5QkbPLXKbLP1J");

#[program]
pub mod anchor_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
