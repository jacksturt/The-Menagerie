use anchor_lang::prelude::*;

#[account]
pub struct BackerData {
    pub backer_pk: Pubkey,
    pub backer_amount: u64,
}

impl Space for BackerData {
    pub INIT_SPACE: usize = 
        8 + // ACCOUNT DISCRIMINATOR
        32 + // BACKER PK
        8; // BACKER AMOUNT
}
