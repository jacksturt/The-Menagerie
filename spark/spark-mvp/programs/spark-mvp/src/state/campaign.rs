use anchor_lang::prelude::*;

#[account]
pub struct Campaign {
    pub campaign_seed: u64,
    pub creator: Pubkey,
    pub started_at: i64,
    pub ending_at: i64,
    pub funding_goal: u64,
    pub is_finished: bool,
    pub campaign_bump: u8,
}

impl Space for Campaign {
    const INIT_SPACE: usize = 
        8 + // ACCOUNT DISCRIMINATOR
        8 + // CAMPAIGN SEED
        32 + // CREATOR
        8 + // STARTED AT
        8 + // ENDING AT
        8 + // FUNDING GOAL
        1 + // IS FINISHED
        1; // CAMPAIGN BUMP 
}
