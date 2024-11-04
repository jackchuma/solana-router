use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;

declare_id!("3QSSxzUPrct8Go38smbPdb8Lo8WnRNyFa6owKnCaYbuW");

#[program]
pub mod simple_router {
    use super::*;

    pub fn route(ctx: Context<Router>, accounts: Vec<TransactionAccount>, call: Call) -> Result<()> {
        let remaining_accounts = &ctx.remaining_accounts;
        let mut account_metas = Vec::with_capacity(accounts.len());
        for acc in accounts.iter() {
            account_metas.push(AccountMeta::from(acc));
        }

        // Build program instruction
        let ix = Instruction {
            program_id: call.to,
            accounts: account_metas.clone(),
            data: call.data.clone(),
        };

        // Make call
        invoke(&ix, remaining_accounts)?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(accounts: Vec<TransactionAccount>, call: Call)]
pub struct Router {}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Call {
    pub to: Pubkey,
    pub data: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

impl From<&TransactionAccount> for AccountMeta {
    fn from(account: &TransactionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}