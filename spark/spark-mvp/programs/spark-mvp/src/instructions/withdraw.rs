use crate::error::SparkError;
use crate::state::Campaign;
use anchor_lang::solana_program::native_token::lamports_to_sol;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        constraint = creator.key() == campaign.creator @ SparkError::UnauthorizedCreator
    )]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"campaign",
            campaign.campaign_seed.to_le_bytes().as_ref(),
            campaign.creator.as_ref()
        ],
        bump = campaign.campaign_bump
    )]
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        require!(
            lamports_to_sol(self.campaign.to_account_info().lamports())
                >= self.campaign.funding_goal as f64,
            SparkError::CampaignFailedNotEnoughFunds
        );
        require!(
            Clock::get()?.unix_timestamp > self.campaign.ending_at,
            SparkError::CampaignStillRunning
        );

        let campaign_seeds = self.campaign.campaign_seed.to_le_bytes();

        let campaign_signer_seeds: &[&[&[u8]]] = &[&[
            b"campaign",
            campaign_seeds.as_ref(),
            self.campaign.creator.as_ref(),
            &[self.campaign.campaign_bump],
        ]];

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.campaign.to_account_info(),
            to: self.creator.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, campaign_signer_seeds);

        transfer(cpi_ctx, self.campaign.to_account_info().lamports())
    }
}
