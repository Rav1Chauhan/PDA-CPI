use anchor_lang::prelude::*;

declare_id!("c9MWmAhSz1Ykw5fxUeCUunRznoe8RFfrow3vjGGmiG9");

#[program]
pub mod pda_cpi {
    use super::*;

    pub fn create_pda_account(ctx: Context<CreatedPdaAccount>) -> Result<()> {
        let pda_account = &mut ctx.accounts.pda_account;
        pda_account.authority = ctx.accounts.payer.key();
        pda_account.amount = 0;
        pda_account.bump = ctx.bumps.pda_account;
        msg!("PDA account created successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatedPdaAccount<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer=payer,
        space=8+32+8+1,
        seeds=[b"client1",payer.key().as_ref()],
        bump
    )]
    pub pda_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}
#[account]
pub struct StakeAccount {
    pub authority: Pubkey,
    pub amount: u64,
    pub bump: u8,
}
