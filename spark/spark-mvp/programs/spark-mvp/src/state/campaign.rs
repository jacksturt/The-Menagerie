pub use state::*;

use crate::state;
use anchor_lang::prelude::*;
#[account]
pub struct Campaign {
    pub campaign_seed: u64,
    pub creator: Pubkey,
    pub started_at: i64,
    pub ending_at: i64,
    pub funding_goal: u64,
    pub finished: bool,
}

impl Campaign {
    pub const SPACE: usize = 
        8 + // ACCOUNT DISCRIMINATOR
        8 + // CAMPAIGN SEED
        32 + // CREATOR
        8 + // STARTED AT
        8 + // ENDING AT
        8 + // FUNDING GOAL
        1; // FINISHED
}
