//- Implement a user profile system with nested data structures

use anchor_lang::prelude::*;

declare_id!("CpGL6WEUbq84Tfv5j2ZKNRtQqayLEhDEoPbXkfrGMAW2");

#[program]
pub mod user_profile {
    use super::*;

    pub fn initialize_profile(ctx: Context<InitializeProfile>, name: String, bio: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        let clock = Clock::get()?;

        profile.owner = ctx.accounts.owner.key();
        profile.username = name;
        profile.bio = bio;

        profile.state = ActiveStatus{
            active: true
        };

        profile.details = BlockDetails {
            blockslot: clock.slot,
            blocktimestamp: clock.unix_timestamp,
        };

        profile.bump = ctx.bumps.profile;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
#[instruction(bio: String)]
pub struct InitializeProfile<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [b"profile", owner.key.as_ref()],
        bump,
    )]
    pub profile: Account<'info, UserProfile>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl UserProfile {
    pub const MAX_USERNAME: usize = 50;
    pub const MAX_BIO: usize = 250;

    pub const INIT_SPACE: usize = 
    32 +
    4 + Self::MAX_USERNAME +
    4 + Self::MAX_BIO +
    1 + 1 +
    8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ActiveStatus {
    pub active: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BlockDetails {
    pub blockslot: u64,
    pub blocktimestamp: i64,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub username: String,
    pub bio: String, // can be updated hence mut
    pub state: ActiveStatus, // can be updated hence mut
    pub details: BlockDetails, // can be updated hence mut
    pub bump: u8,
}
/*

nested structure of user profile

profile has
>> name >> immutable
>> user last block number >>  mut
>> user last time-stamp >> mut
>> user bio >> mut
>> user active state >> nested struct active >> active or not active
>> user block details >> nested struct >> block number and block timestamp




*/