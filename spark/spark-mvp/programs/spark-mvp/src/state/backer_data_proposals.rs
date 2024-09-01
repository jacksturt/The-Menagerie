pub use state::*;

use crate::state;
use anchor_lang::prelude::*;
#[account]
pub struct BackerDataProposals {
    pub backer_pk_proposals: Pubkey,
    pub voting_power: Option<u64>,
    pub voted: bool,
}

impl BackerDataProposals {
    pub const SPACE: usize = 8 + 32 + 8 + 8 + 1;
}
