use crate::{account::*};
use anchor_lang::prelude::*;

use crate::constants::*;

pub fn fn_dev_fee(pool: &Pool, amount: u64) -> Result<u64> {
    let fee = pool.dev_fee as u128;
    let res = (amount as u128) * fee / 100 / PERCENT_MULTIPLIER as u128;
    Ok(res as u64)
}

pub fn fn_burn_fee(pool: &Pool, amount: u64) -> Result<u64> {
    let fee = pool.burn_fee as u128;
    let res = (amount as u128) * fee / 100 / PERCENT_MULTIPLIER as u128;
    Ok(res as u64)
}
