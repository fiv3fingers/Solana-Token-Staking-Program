use crate::errors::TapestrySackingError;
use crate::{constants::CONFIG, state::config::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Pause<'info> {
    #[account(
        mut,
        constraint = global_config.authority == admin.key() @TapestrySackingError::IncorrectAuthority
    )]
    admin: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,
}

impl<'info> Pause<'info> {
    pub fn handler(&mut self, is_stop: u8) -> Result<()> {
        let global_config = &mut self.global_config;

        msg!("is_stop: {:?}", is_stop);
        global_config.is_stop = is_stop;

        msg!("global_config: {:#?}", global_config);

        Ok(())
    }
}
