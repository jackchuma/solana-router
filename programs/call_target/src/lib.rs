use anchor_lang::prelude::*;

declare_id!("82UAtJWsPJvmD6JV1Z3bpL4Nr6TD7gQrZNN9K3HMygV6");

#[program]
pub mod call_target {
    use super::*;

    pub fn make_call(ctx: Context<MakeCall>, _data: Vec<u8>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_data: Vec<u8>)]
pub struct MakeCall<'info> {
    pub caller: Signer<'info>,
}
