use anchor_lang::prelude::*;

use crate::state::Campaign;

#[derive(Accounts)]
#[instruction(campaign_seed: u64)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Campaign::INIT_SPACE,
        seeds = [
            b"campaign",
            campaign_seed.to_le_bytes().as_ref(),
            creator.key().as_ref()
        ],
        bump
    )]
    pub campaign: Account<'info, Campaign>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCampaign>, campaign_seed: u64, ending_at: i64, funding_goal: u64) -> Result<()> {
    ctx.accounts.create_campaign(campaign_seed, ending_at, funding_goal, &ctx.bumps)?;

    Ok(())
}

impl<'info> CreateCampaign<'info> {
    pub fn create_campaign(
        &mut self,
        campaign_seed: u64,
        ending_at: i64,
        funding_goal: u64,
        bumps: &CreateCampaignBumps
    ) -> Result<()> {
        self.campaign.set_inner(Campaign {
            campaign_seed,
            creator: self.creator.key(),
            started_at: Clock::get()?.unix_timestamp,
            ending_at,
            funding_goal,
            is_finished: false,
            campaign_bump: bumps.campaign,
        });

        Ok(())
    }
}
