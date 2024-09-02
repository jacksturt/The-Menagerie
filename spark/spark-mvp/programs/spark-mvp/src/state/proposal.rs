use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub votes_for: u64,
    pub votes_against: u64,
    pub started_at: i64,
    pub ending_at: i64,
    pub proposal_passed: Option<bool>,
    pub proposal_bump: u8,
}

impl Space for Proposal {
    const INIT_SPACE: usize = 
        8 + // ACCOUNT DISCRIMINATOR
        8 + // VOTES FOR
        8 + // VOTES AGAINST
        8 + // STARTED AT
        8 + // ENDING AT
        1 + // PROPOSAL PASSED
        1;  // PROPOSAL BUMP
}
