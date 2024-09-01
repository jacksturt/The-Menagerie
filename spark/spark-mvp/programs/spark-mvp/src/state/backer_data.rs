pub use state::*;

use crate::state;
use anchor_lang::prelude::*;
#[account]
pub struct BackerData {
    pub backer_pk: Pubkey,
    pub backer_amount: u64,
}

impl BackerData {
    pub const SPACE: usize = 8 + 32 + 8;
}
