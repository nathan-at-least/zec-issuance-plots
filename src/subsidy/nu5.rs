//! Model the block subsidy for a given height for Zcash mainnet as of NU5.
//!
//! WARNING: This model has not yet been verified against the zcashd source code, which it is
//! transcribed from.

use crate::consts::{
    blossom_activation, blossom_pow_target_spacing_ratio, post_blossom_halving_interval,
    pre_blossom_halving_interval, start_subsidy, subsidy_slow_start_interval,
    subsidy_slow_start_shift,
};
use crate::units::{Halvings, Height, Zec};

pub fn max_supply() -> Zec {
    let mut m: Zec = 0;

    // Loop body logic must skip h == 0 -> 0 ZAT.
    for h in 1.. {
        let subsidy = block_subsidy(h);
        if subsidy == 0 {
            return m;
        } else {
            m += subsidy;
        }
    }
    unreachable!();
}

/// Transcription of `zcash/src/main.cpp` `GetBlockSubsidy`
pub fn block_subsidy(height: Height) -> Zec {
    if height < subsidy_slow_start_shift() {
        return (start_subsidy() / subsidy_slow_start_interval()) * height;
    } else if height < subsidy_slow_start_interval() {
        return (start_subsidy() / subsidy_slow_start_interval()) * (height + 1);
    }

    let halvings = halvings_at(height);
    if halvings > 63 {
        0
    } else if height >= blossom_activation() {
        (start_subsidy() / blossom_pow_target_spacing_ratio()) >> halvings
    } else {
        start_subsidy() >> halvings
    }
}

// Transcription of `zcash/src/consensus/params.cpp` `Params::Halving`
fn halvings_at(height: Height) -> Halvings {
    // BUG This case not handled in zcashd!
    if height < subsidy_slow_start_shift() {
        0
    } else if height >= blossom_activation() {
        // The number of blocks between the end of the shift and blossom activation:
        let post_shift_pre_blossom: Height = blossom_activation() - subsidy_slow_start_shift();

        // The "scaled" pseudo-number of pre_blossom blocks:
        // A more precise typing would distinguish "pre/post" blossom blocks/heights.
        let pre_blossom_adjusted: Height =
            post_shift_pre_blossom * blossom_pow_target_spacing_ratio();

        // The number of blocks post blossom:
        let post_blossom: Height = height - blossom_activation();

        // The number of "scaled blocks" (post slow start shift):
        let scaled_halvings: Height = pre_blossom_adjusted + post_blossom;

        // The number of post-blossom halving intervals for "blossom-scaled" height:
        scaled_halvings / post_blossom_halving_interval()
    } else {
        (height - subsidy_slow_start_shift()) / pre_blossom_halving_interval()
    }
}

#[cfg(test)]
mod tests;
