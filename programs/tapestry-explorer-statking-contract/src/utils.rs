use crate::*;
use anchor_spl::token::{self, Token};
use std::ops::{Div, Mul};

pub fn convert_to_float(value: u64, decimals: u8) -> f64 {
    (value as f64).div(f64::powf(10.0, decimals as f64))
}

pub fn convert_from_float(value: f64, decimals: u8) -> u64 {
    value.mul(f64::powf(10.0, decimals as f64)) as u64
}

//  transfer token from user
pub fn token_transfer_user<'info>(
    from: AccountInfo<'info>,
    authority: &Signer<'info>,
    to: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new(
        token_program.to_account_info(),
        token::Transfer {
            from,
            authority: authority.to_account_info(),
            to,
        },
    );
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

//  transfer token from PDA
pub fn token_transfer_with_signer<'info>(
    from: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    to: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new_with_signer(
        token_program.to_account_info(),
        token::Transfer {
            from,
            to,
            authority,
        },
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    Ok(())
}
