/* 
The users SHOULD not be able to cancel or change their pledge, 
however they can request from the campaign creator for a refund, 
and this is the instruction used to refund users 
*/

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct RefundPledge<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    // #[account(
    //     mut,
    //     seeds = [
    //         b"campaign",
    //         campaign.campaign_seed.to_le_bytes().as_ref(),
    //         creator.key().as_ref()
    //     ],
    //     bump = campaign.campaign_bump,
    // )]
    // pub campaign: Account<'info, Campaign>,

    #[account(
        mut
    )]
    pub backer_data: Account<'info, BackerData>,

    #[account(mut)]
    pub treasury: Account<'info, TokenAccount>,

    #[account(mut)]
    pub backer_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateProposal<'info> {
    pub fn refund_backer(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.backer_token_account.to_account_info(),
            authority: self.creator.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.backer_data.backer_amount)?;

        Ok(())
    }
}