use anchor_lang::prelude::*;
pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    accept_authority::*, claim_reward::*, configure::*, deposit_fund::*, nominate_authority::*,
    pause::*, stake::*, unstake::*,
};

use state::config::*;

declare_id!("H799pQrKs8W84t6svhBog62tWSHjCNLQxpH41J6RUZ8z");

#[program]
pub mod tapestry_explorer_statking_contract {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    //  called by admin to set global config
    //  need to check the signer is authority
    pub fn configure(ctx: Context<Configure>, new_config: Config) -> Result<()> {
        msg!("configure: {:#?}", new_config);
        ctx.accounts.handler(new_config, ctx.bumps.config)
    }

    //  Admin can hand over admin role
    pub fn nominate_authority(ctx: Context<NominateAuthority>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.process(new_admin)
    }

    //  Pending admin should accept the admin role
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        ctx.accounts.process()
    }

    //deposit the fund
    pub fn deposit_fund(ctx: Context<DepositFund>) -> Result<()> {
        ctx.accounts.process_instruction()
    }

    //user stake tokens
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.process_instruction(amount)
    }

    //user withdreaw tokens
    pub fn unstake(ctx: Context<UnStake>) -> Result<()> {
        ctx.accounts.process_instruction(ctx.bumps.global_config)
    }

    //user claim reward
    pub fn claim_reward(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.process_instruction(ctx.bumps.global_config)
    }

    //Admin pause the Contract
    pub fn pause(ctx: Context<Pause>, is_stop: u8) -> Result<()> {
        msg!("pause: is_stop: {:?}", is_stop);
        ctx.accounts.handler(is_stop)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
