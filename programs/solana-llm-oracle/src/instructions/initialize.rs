use anchor_lang::prelude::*;

use crate::Config;

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Sol.College lab patch: admin is the lab deployer (upstream grvFMyb...).
    #[account(mut, address = pubkey!("BP1uwSaTHXvgN8sPQJgaor6iBoT3KYjnZNnkvJr71zDv"))]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}

impl Initialize<'_> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.config.set_inner(Config { bump: bumps.config });
        Ok(())
    }
}
