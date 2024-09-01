pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("33RrYnFPia7wguALxCCA4inj4LQBdo7QVacKZiQ2i1nZ");

#[program]
pub mod spark_mvp {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, campaign_seed: u64, ending_at: i64, funding_goal: u64) -> Result<()> {
        create_campaign::handler(campaign_seed, ending_at, funding_goal)?;
    }
}
