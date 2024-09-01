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

    pub fn create_campaign(ctx: Context<CreateCampaign>) -> Result<()> {
        todo!()
    }
}
