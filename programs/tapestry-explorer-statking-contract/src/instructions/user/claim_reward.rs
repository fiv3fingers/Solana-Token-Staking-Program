use crate::*;

use crate::{
    constants::{CONFIG, USERINFO},
    errors::*,
    state::{config::*, user::*},
};
use anchor_lang::{ system_program};
use anchor_spl::{
    token::{self,  Token, TokenAccount},
};

use crate::utils::*;

#[derive(Accounts)]
pub struct Claim<'info> {
    
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(
        mut,
        token::mint = global_config.token_mint_config,
        token::authority = global_config,
    )]
    pub global_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [USERINFO.as_bytes(), &user.key().to_bytes()],
        bump
    )]
    pub user_info: Box<Account<'info, User>>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
        mut, 
        token::mint = global_config.token_mint_config, 
        token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> Claim<'info> {
    pub fn process_instruction(&mut self, global_config_bump: u8) -> Result<()> {
        
        let timestamp = Clock::get()?.unix_timestamp;
        let global_config = &mut self.global_config;
        let user_token_account = &mut self.user_token_account;
        let global_token_account = &mut self.global_token_account;
        let user_info = &mut self.user_info;
        
        require!(global_config.is_stop == 0, TapestrySackingError::ContractIsStopped);
        msg!("deposit: {:?}, debt: {:?}, timestamp: {:?}", user_info.deposit, user_info.debt, timestamp);
        // calc the reward token amount
        let claim_amt = global_config.calc_reaward(user_info.deposit, user_info.debt, timestamp)?;
        msg!("claim_amt: {:?}", claim_amt);

        let signer_seeds: &[&[&[u8]]] = &[&[
            CONFIG.as_bytes(),
            &[global_config_bump],
        ]];

        // transfer token to user
        token_transfer_with_signer(
            global_token_account.to_account_info(),
            global_config.to_account_info(),
            user_token_account.to_account_info(),
            &self.token_program,
            signer_seeds,
            claim_amt,
        )?;
        user_info.debt = user_info.debt.checked_add(claim_amt).ok_or(TapestrySackingError::InvalidArgument)?;

        Ok(())
    }
}
