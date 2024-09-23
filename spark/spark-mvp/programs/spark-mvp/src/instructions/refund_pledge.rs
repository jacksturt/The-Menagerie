/* 
The users SHOULD not be able to cancel or change their pledge, 
however they can request from the campaign creator for a refund, 
and this is the instruction used to refund users 
*/

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
// use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct RefundPledge<'info> {
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
        mut,
        seeds = [
            "backer-data".as_bytes(),
            backer.key().as_ref(),
            campaign.campaign_seed.to_le_bytes().as_ref()],
        space = BackerData::INIT_SPACE,
        bump,
    )]
    pub backer_data: Account<'info, BackerData>,

    // #[account(mut)]
    // pub treasury: Account<'info, TokenAccount>,

    // #[account(mut)]
    // pub backer_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposal<'info> {
    pub fn refund_backer(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.campaign.to_account_info(),
            to: self.backer_data.backer_pk.to_account_info(),
        };

        let campaign_seeds = self.campaign.campaign_seed.to_le_bytes();

        let campaign_signer_seeds: &[&[&[u8]]] = &[&[
            b"campaign",
            campaign_seeds.as_ref(),
            self.campaign.creator.as_ref(),
            &[self.campaign.campaign_bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, campaign_signer_seeds);

        let backed_amount = self.backer_data.backer_amount.checked_mul(LAMPORTS_PER_SOL).unwrap();

        transfer(cpi_ctx, backed_amount)?;

        Ok(())
    }
}