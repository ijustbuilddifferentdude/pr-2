use anchor_lang::prelude::*;

declare_id!("AfgNW3h4kkE6GW8T2c3JxxropMLPTsQjhJMPQJqH4VbZ");

#[program]
pub mod pr_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let ctf_state = &mut ctx.accounts.ctf_state;
        ctf_state.admin = *ctx.accounts.admin.key;
        ctf_state.score = 0;
        ctf_state.flag_claimed = false;
        Ok(())
    }

    pub fn update_score(ctx: Context<UpdateScore>, points: u64) -> Result<()> {
        let ctf_state = &mut ctx.accounts.ctf_state;
        ctf_state.score = ctf_state.score.saturating_add(points);
        Ok(())
    }

    pub fn claim_flag(ctx: Context<ClaimFlag>) -> Result<()> {
        let ctf_state = &mut ctx.accounts.ctf_state;
        require!(ctf_state.score >= 100, CtfError::NotEnoughPoints);
        ctf_state.flag_claimed = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + 32 + 8 + 1,
        seeds = [b"ctf_state", admin.key.as_ref()], 
        bump
    )]
    pub ctf_state: Account<'info, CtfState>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateScore<'info> {
    #[account(mut, seeds = [b"ctf_state", ctf_state.admin.as_ref()], bump)]
    pub ctf_state: Account<'info, CtfState>,
}

#[derive(Accounts)]
pub struct ClaimFlag<'info> {
    #[account(mut, seeds = [b"ctf_state", ctf_state.admin.as_ref()], bump)]
    pub ctf_state: Account<'info, CtfState>,
}

#[account]
pub struct CtfState {
    pub admin: Pubkey,
    pub score: u64,
    pub flag_claimed: bool,
}

#[error_code]
pub enum CtfError {
    #[msg("Not enough points")]
    NotEnoughPoints,
}
