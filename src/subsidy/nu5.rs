//! Model the block subsidy for a given height for Zcash mainnet as of NU5.
//!
//! WARNING: This model has not yet been verified against the zcashd source code, which it is
//! transcribed from.

use crate::consts::{
    BLOSSOM_ACTIVATION, BLOSSOM_POW_TARGET_SPACING_RATIO, POST_BLOSSOM_HALVING_INTERVAL,
    PRE_BLOSSOM_HALVING_INTERVAL, START_SUBSIDY, SUBSIDY_SLOW_START_INTERVAL,
    SUBSIDY_SLOW_START_SHIFT,
};
use crate::units::{Height, Zat};

/// Transcription of `zcash/src/main.cpp` `GetBlockSubsidy`
pub fn block_subsidy(height: Height) -> Zat {
    if height < SUBSIDY_SLOW_START_SHIFT {
        return (START_SUBSIDY / SUBSIDY_SLOW_START_INTERVAL) * height;
    } else if height < SUBSIDY_SLOW_START_INTERVAL {
        return (START_SUBSIDY / SUBSIDY_SLOW_START_INTERVAL) * (height + 1);
    }

    let halvings = halvings_at(height);
    if height >= BLOSSOM_ACTIVATION {
        (START_SUBSIDY / BLOSSOM_POW_TARGET_SPACING_RATIO) >> halvings
    } else {
        START_SUBSIDY >> halvings
    }
}

// Transcription of `zcash/src/consensus/params.cpp` `Params::Halving`
fn halvings_at(height: Height) -> usize {
    // BUG This case not handled in zcashd!
    if height < SUBSIDY_SLOW_START_SHIFT {
        0
    } else if height >= BLOSSOM_ACTIVATION {
        // The number of blocks between the end of the shift and blossom activation:
        let post_shift_pre_blossom: Height = BLOSSOM_ACTIVATION - SUBSIDY_SLOW_START_SHIFT;

        // The "scaled" pseudo-number of pre_blossom blocks:
        // A more precise typing would distinguish "pre/post" blossom blocks/heights.
        let pre_blossom_adjusted: Height =
            post_shift_pre_blossom * BLOSSOM_POW_TARGET_SPACING_RATIO;

        // The number of blocks post blossom:
        let post_blossom: Height = height - BLOSSOM_ACTIVATION;

        // The number of "scaled blocks" (post slow start shift):
        let scaled_halvings: Height = pre_blossom_adjusted + post_blossom;

        // The number of post-blossom halving intervals for "blossom-scaled" height:
        scaled_halvings / POST_BLOSSOM_HALVING_INTERVAL
    } else {
        (height - SUBSIDY_SLOW_START_SHIFT) / PRE_BLOSSOM_HALVING_INTERVAL
    }
}

#[cfg(test)]
mod tests;
