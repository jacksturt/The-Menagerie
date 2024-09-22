use anchor_lang::prelude::*;

#[error_code]
pub enum SparkError {
    #[msg("Pledge Amount Can't Be Zero")]
    PledgeAmountZero,
    #[msg("Campaign Has Been Finished")]
    CampaignHasFinished,
    #[msg("Campaign Is Still Running")]
    CampaignStillRunning,
    #[msg("Unauthorized Campaign Creator As a Signer")]
    UnauthorizedCreator,
    #[msg("Campaign Failed Due To Not Raising Enough Funds")]
    CampaignFailedNotEnoughFunds,
}
