pub use state::*;

use crate::state;
use anchor_lang::prelude::*;
#[account]
pub struct Proposal {
    pub votes_for: u64,
    pub votes_against: Pubkey,
    pub started_at: i64,
    pub ending_at: i64,
    pub proposal_passed: Option<bool>,
}

impl Proposal {
    pub const SPACE: usize = 8 + 8 + 32 + 8 + 8 + 8 + 1;
}
