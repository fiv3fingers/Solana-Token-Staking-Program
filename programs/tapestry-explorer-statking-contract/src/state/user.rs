use anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize};
use core::fmt::Debug;

#[account]
#[derive(Default, Debug)]
pub struct User {
    /// user publickey
    pub user: Pubkey,

    /// deposted stake token amount
    pub deposit: u64,

    /// withdrawed Reward Token amount
    pub debt: u64,

    /// the latest staking date
    pub last_update: i64,

    // flag to check if user is already staker
    pub initialized: u8,
}

impl User {
    pub const DATA_SIZE: usize = 8 + 32 + 8 + 8 + 8 + 1;
}
