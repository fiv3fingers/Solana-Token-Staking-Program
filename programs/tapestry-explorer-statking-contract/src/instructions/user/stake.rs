use crate::*;

use crate::{
    constants::{CONFIG, USERINFO},
    errors::*,
    state::{config::*, user::*},
};
use anchor_lang::system_program;

use crate::utils::*;
use anchor_spl::{
    associated_token::{self},
    token::{self, Token, TokenAccount},
};

use std::ops::Mul;

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [CONFIG.as_bytes()],
        bump,
    )]
    global_config: Box<Account<'info, Config>>,

    /// CHECK: ata of global vault
    #[account(
        mut,
        seeds = [
            global_config.key().as_ref(),
            token::spl_token::ID.as_ref(),
            global_config.token_mint_config.key().as_ref(),
        ],
        bump,
        seeds::program = associated_token::ID
    )]
    pub global_token_account: AccountInfo<'info>,

    // #[account(
    //     mut,
    //     token::mint = global_config.token_mint_config,
    //     token::authority = global_config,
    // )]
    // pub global_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        space = User::DATA_SIZE,
        seeds = [USERINFO.as_bytes(), &user.key().to_bytes()],
        bump
    )]
    pub user_info: Box<Account<'info, User>>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
        mut,
        constraint = user_token_account.mint == global_config.token_mint_config,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.amount >= amount,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn process_instruction(&mut self, amount: u64) -> Result<()> {
        msg!("Stake: deposit_update start");
        require!(amount > 0, TapestrySackingError::InvalidAmount);

        let timestamp = Clock::get()?.unix_timestamp;
        let user_info = &mut self.user_info;
        let global_config = &mut self.global_config;
        let origin_deposit = user_info.deposit;

        require!(
            global_config.is_stop == 0,
            TapestrySackingError::ContractIsStopped
        );
        //init user info PDA
        if user_info.initialized == 0 {
            user_info.user = self.user.key();
            user_info.initialized = 1;
        }
        require!(
            timestamp - user_info.last_update >= global_config.claim_period,
            TapestrySackingError::NeedMoreTimeToDeposit
        );
        user_info.last_update = timestamp;

        msg!("Stake: deposit_update before");

        global_config.deposit_update(timestamp)?;

        msg!("Stake: deposit_update after");

        if origin_deposit == 0 {
            global_config.total_stakers = global_config
                .total_stakers
                .checked_add(1)
                .ok_or(TapestrySackingError::InvalidArgument)?;
        }

        let user_token_account = &mut self.user_token_account;
        let global_token_account = &mut self.global_token_account;
        let user = &mut self.user;

        //transfer token
        token_transfer_user(
            user_token_account.to_account_info(),
            &user,
            global_token_account.to_account_info(),
            &self.token_program,
            amount,
        )?;

        //update user info pda
        let user_info = &mut self.user_info;

        user_info.deposit = user_info
            .deposit
            .checked_add(amount)
            .ok_or(TapestrySackingError::InvalidArgument)?;
        global_config.total_deposit = global_config
            .total_deposit
            .checked_add(amount)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        let change_amount = user_info
            .deposit
            .checked_sub(origin_deposit)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        user_info.debt = user_info
            .debt
            .checked_add(convert_from_float(
                convert_to_float(change_amount, 6).mul(global_config.total_rate),
                6,
            ))
            .ok_or(TapestrySackingError::InvalidArgument)?;
        Ok(())
    }
}
