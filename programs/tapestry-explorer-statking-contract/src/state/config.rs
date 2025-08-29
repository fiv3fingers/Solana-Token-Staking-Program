use crate::errors::TapestrySackingError;
use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use core::fmt::Debug;

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;

use crate::utils::*;

#[account]
#[derive(Debug)]
pub struct Config {
    /// authority publickey
    pub authority: Pubkey,

    //  use this for 2 step ownership transfer
    pub pending_authority: Pubkey,

    /// stake token mint publickey
    pub token_mint_config: Pubkey,

    /// reward token claim period
    pub claim_period: i64,

    pub total_rate: f64,

    /// number of stakers
    pub total_stakers: u64,

    /// the latest staking time
    pub last_reward_time: i64,

    /// reward for one sec
    pub reward_multiplier: u64,

    /// reward token deposit time
    pub deposit_time: i64,

    /// total stakeed token amount
    pub total_deposit: u64,

    /// reward token amount
    pub purchase_amt: u64,

    /// is stopped
    pub is_stop: u8,

    pub initialized: bool,
}

impl Config {
    pub fn get_multiplier(&self, from: i64, to: i64) -> Result<u64> {
        msg!("get_multiplier:: from: {:?} to: {:?}", from, to);
        require!(to >= from, TapestrySackingError::ValueInvalid);

        let duration = to
            .checked_sub(from)
            .ok_or(TapestrySackingError::InvalidArgument)? as u64;
        let result = duration
            .checked_mul(self.reward_multiplier)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        msg!(
            "get_multiplier:: duration: {:?} self.reward_multiplier: {:?} result: {:?}",
            duration,
            self.reward_multiplier,
            result
        );
        Ok(result)
    }

    pub fn deposit_update(&mut self, time_stamp: i64) -> Result<()> {
        msg!(
            "deposit_update: time_stamp: {:?} self.last_reward_time:{:?}",
            time_stamp,
            self.last_reward_time
        );

        if time_stamp <= self.last_reward_time {
            return Ok(());
        }

        if self.total_deposit == 0 {
            self.last_reward_time = time_stamp;
            return Ok(());
        }

        let n_time_stamp = time_stamp
            .checked_div(self.claim_period)
            .ok_or(TapestrySackingError::InvalidArgument)?
            .checked_mul(self.claim_period)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        msg!(
            "deposit_update: n_time_stamp: {:?} self.last_reward_time: {:?}",
            n_time_stamp,
            self.last_reward_time
        );

        let n_multiplier = self.get_multiplier(self.last_reward_time, n_time_stamp)?;

        msg!(
            "deposit_update: n_multiplier: {:?} self.last_reward_time: {:?} n_time_stamp: {:?}",
            n_multiplier,
            self.last_reward_time,
            n_time_stamp
        );

        self.total_rate = self
            .total_rate
            .add((n_multiplier as f64).div(self.total_deposit as f64));

        msg!(
            "deposit_update: self.total_rate: {:?} n_multiplier: {:?} self.total_deposit: {:?}",
            self.total_rate,
            n_multiplier,
            self.total_deposit
        );

        self.last_reward_time = n_time_stamp;

        msg!(
            "withdraw_update:: self.last_reward_time: {:?}",
            self.last_reward_time
        );

        Ok(())
    }

    pub fn withdraw_update(&mut self, time_stamp: i64) -> Result<()> {
        let n_time_stamp = time_stamp
            .checked_div(self.claim_period)
            .ok_or(TapestrySackingError::InvalidArgument)?
            .checked_mul(self.claim_period)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        msg!(
            "withdraw_update:: n_time_stamp: {:?} time_stamp: {:?}",
            n_time_stamp,
            time_stamp
        );

        let n_multiplier = self.get_multiplier(self.last_reward_time, n_time_stamp)?;

        msg!(
            "withdraw_update:: n_multiplier: {:?} self.last_reward_time: {:?} n_time_stamp: {:?}",
            n_multiplier,
            self.last_reward_time,
            n_time_stamp
        );

        self.total_rate = self
            .total_rate
            .add((n_multiplier as f64).div(self.total_deposit as f64));

        msg!(
            "withdraw_update:: self.total_rate: {:?} n_multiplier: {:?} self.total_deposit: {:?}",
            self.total_rate,
            n_multiplier,
            self.total_deposit
        );

        self.last_reward_time = n_time_stamp;

        msg!(
            "withdraw_update:: self.last_reward_time: {:?}",
            self.last_reward_time
        );

        Ok(())
    }

    pub fn calc_reaward(
        &mut self,
        user_deposit: u64,
        user_debt: u64,
        time_stamp: i64,
    ) -> Result<u64> {
        let mut acc_per_share: f64 = self.total_rate;

        msg!(
            "calc_reaward: acc_per_share: {:?} self.total_rate: {:?}",
            acc_per_share,
            self.total_rate
        );

        let n_time_stamp = time_stamp
            .checked_div(self.claim_period)
            .ok_or(TapestrySackingError::InvalidArgument)?
            .checked_mul(self.claim_period)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        msg!(
            "n_time_stamp: {:?}, self.claim_period: {:?}, acc_per_share: {:?}",
            n_time_stamp,
            self.claim_period,
            acc_per_share
        );
        if n_time_stamp > self.last_reward_time && self.total_deposit != 0 {
            let n_multiplier = self.get_multiplier(self.last_reward_time, n_time_stamp)?;
            msg!(
                "n_multiplier: {:?}, self.last_reward_time: {:?}, n_time_stamp: {:?}",
                n_multiplier,
                self.last_reward_time,
                n_time_stamp
            );

            let divisor = (n_multiplier as f64).div(self.total_deposit as f64);
            msg!(
                "divisor: {:?}, n_multiplier: {:?}, self.total_deposit: {:?}",
                divisor,
                n_multiplier,
                self.total_deposit
            );
            acc_per_share = acc_per_share.add(divisor);

            msg!("acc_per_share: {:?}", acc_per_share);
        }
        let result = convert_from_float(convert_to_float(user_deposit, 6).mul(acc_per_share), 6)
            .checked_sub(user_debt)
            .ok_or(TapestrySackingError::InvalidArgument)?;

        msg!(
            "result: {:?}, user_deposit: {:?}, user_debt: {:?}",
            result,
            acc_per_share,
            user_debt
        );
        Ok(result)
    }
}
