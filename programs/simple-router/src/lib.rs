use anchor_lang::prelude::*;

declare_id!("3QSSxzUPrct8Go38smbPdb8Lo8WnRNyFa6owKnCaYbuW");

#[program]
pub mod simple_router {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
