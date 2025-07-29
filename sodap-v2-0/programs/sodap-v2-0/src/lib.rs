use anchor_lang::prelude::*;

declare_id!("AcckWeV1jLhufv9me3wt3ruSrcy1iinSDw1fULLr6xAd");

#[program]
pub mod sodap_v2_0 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
