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
    pub const SPACE: usize = 8 + 8 + 32 + 8 + 8 + 8 + 1;
}
