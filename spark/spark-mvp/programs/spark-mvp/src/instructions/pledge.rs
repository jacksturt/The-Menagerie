use crate::error::SparkError;
use crate::state::{BackerData, Campaign};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct Pledge<'info> {
    #[account(mut)]
    pub backer: Signer<'info>,

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

    #[account(
        init_if_needed,
        payer = backer,
        seeds = [
            "backer-data".as_bytes(),
            backer.key().as_ref(),
            campaign.campaign_seed.to_le_bytes().as_ref()],
        space = 8 + BackerData::INIT_SPACE,
        bump,
    )]
    pub backer_data: Account<'info, BackerData>,
    pub system_program: Program<'info, System>,
}

impl<'info> Pledge<'info> {
    pub fn pledge(&mut self, pledge_amount: u64) -> Result<()> {
        require!(pledge_amount > 0, SparkError::PledgeAmountZero);
        require!(
            Clock::get()?.unix_timestamp < self.campaign.ending_at,
            SparkError::CampaignHasFinished
        );

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.backer.to_account_info(),
            to: self.campaign.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, pledge_amount)?;

        if self.backer_data.backer_amount == 0 {
            self.backer_data.set_inner(BackerData {
                backer_pk: self.backer.key(),
                backer_amount: pledge_amount,
            });

            Ok(())
        } else {
            self.backer_data.backer_amount += pledge_amount;
            Ok(())
        }
    }
}
