use anchor_lang::prelude::*;

#[account]
pub struct BackerData {
    pub backer_pk: Pubkey,
    pub backer_amount: u64,
    pub backer_data_bump: u8,
}

impl Space for BackerData {
    const INIT_SPACE: usize = 
        8 + // ACCOUNT DISCRIMINATOR
        32 + // BACKER PK
        8 + // BACKER AMOUNT
        1; // BACKER BUMP
}
