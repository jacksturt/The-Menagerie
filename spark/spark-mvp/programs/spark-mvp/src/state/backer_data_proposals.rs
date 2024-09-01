use anchor_lang::prelude::*;

#[account]
pub struct BackerDataProposals {
    pub backer_pk_proposals: Pubkey,
    pub voting_power: Option<u64>,
    pub has_voted: bool,
}

impl Space for BackerDataProposals {
    pub INIT_SPACE: usize = 
        8 + // ACCOUNT DISCRIMINATOR
        32 + // BACKER PK PROPOOSALS
        8 + 8 + // <OPTION> VOTING POWER 
        1; // HAS VOTED
}
