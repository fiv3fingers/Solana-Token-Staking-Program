use crate::{
    constants::{CONFIG, WEEK_SECONDS},
    errors::*,
    state::config::*,
};

use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use crate::utils::*;

#[derive(Accounts)]
pub struct DepositFund<'info> {
    #[account(
        mut,
        constraint = global_config.authority == admin.key() @TapestrySackingError::IncorrectAuthority
    )]
    admin: Signer<'info>,

    #[account(mut)]
    invester: Signer<'info>,

    #[account(
        mut,
        constraint = invester_token_account.mint == global_config.token_mint_config @TapestrySackingError::DepositInvesterTokenMintError,
        constraint = invester_token_account.owner == invester.key() @TapestrySackingError::DepositInvesterTokenOwnerError,
        constraint = invester_token_account.amount >= global_config.purchase_amt @TapestrySackingError::DepositInvesterTokenAmountError,
    )]
    pub invester_token_account: Account<'info, TokenAccount>,

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
    pub reward_vault: AccountInfo<'info>,

    #[account(
        constraint = token_mint.key() ==  global_config.token_mint_config @TapestrySackingError::DepositRewardTokenMintError
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DepositFund<'info> {
    pub fn process_instruction(&mut self) -> Result<()> {
        let rewards_vault = &mut self.reward_vault;

        // create rewards_vault pda
        if rewards_vault.data_is_empty() {
            anchor_spl::associated_token::create(CpiContext::new(
                self.associated_token_program.to_account_info(),
                anchor_spl::associated_token::Create {
                    payer: self.admin.to_account_info(),
                    associated_token: self.reward_vault.to_account_info(),
                    authority: self.global_config.to_account_info(),

                    mint: self.token_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                },
            ))?;
        }

        let global_config = &mut self.global_config;
        let timestamp = Clock::get()?.unix_timestamp;

        msg!("Staked Time: {}", timestamp);

        let invester_token_account = &mut self.invester_token_account;
        let reward_vault = &mut self.reward_vault;
        let invester = &mut self.invester;

        // Transfers tokens from the user's wallet to the contract account.
        token_transfer_user(
            invester_token_account.to_account_info(),
            &invester,
            reward_vault.to_account_info(),
            &self.token_program,
            global_config.purchase_amt,
        )?;

        // update global_config pda
        global_config.reward_multiplier = global_config.purchase_amt / 2 / WEEK_SECONDS;
        global_config.deposit_time = timestamp;

        msg!("global_config: {:#?}", global_config);
        Ok(())
    }
}
