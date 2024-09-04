use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(proposal_seed: u64)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"campaign",
            campaign.campaign_seed.to_le_bytes().as_ref(),
            creator.key().as_ref()
        ],
        bump = campaign.campaign_bump,
    )]
    pub campaign: Account<'info, Campaign>,

    #[account(
        init,
        payer = creator,
        space = Proposal::INIT_SPACE,
        seeds = [
            b"proposal",
            proposal_seed.to_le_bytes().as_ref(),
            campaign.key().as_ref(),
        ],
        bump
    )]
    pub proposal: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposal<'info> {
    pub fn create_proposal(&mut self, ending_at: i64, bumps: &CreateProposalBumps) -> Result<()> {
        self.proposal.set_inner(Proposal {
            votes_for: 0,
            votes_against: 0,
            started_at: Clock::get()?.unix_timestamp,
            ending_at,
            proposal_passed: None,
            proposal_bump: bumps.proposal,
        });

        Ok(())
    }
}
