//! Model the block subsidy for a given height for Zcash mainnet as of NU5.
//!
//! WARNING: This model has not yet been verified against the zcashd source code, which it is
//! transcribed from.

use crate::units::{Height, Zat};

// Consts are pub primarily so `cargo doc` shows them.
/// Transcription from `zcash/src/amount.h`
pub const COIN: Zat = 100_000_000; // Number of Zatoshi per ZEC

/// Transcription from `zcash/src/main.cpp`
pub const START_SUBSIDY: Zat = COIN * 125 / 10; // 12.5 ZEC

/// Transcription from `zcash/src/chainparams.cpp`
pub const SUBSIDY_SLOW_START_INTERVAL: Height = 20_000;

/// Transcription from `zcash/src/chainparams.cpp`
pub const BLOSSOM_ACTIVATION: Height = 653_600;

/// Transcription from `zcash/src/consensus/params.h`
pub const BLOSSOM_POW_TARGET_SPACING_RATIO: usize = {
    // Hide these constants which are only used to calculate the ratio:
    pub const PRE_BLOSSOM_POW_TARGET_SPACING: usize = 150;
    pub const POST_BLOSSOM_POW_TARGET_SPACING: usize = 75;

    PRE_BLOSSOM_POW_TARGET_SPACING / POST_BLOSSOM_POW_TARGET_SPACING
};

/// Transcription from `zcash/src/consensus/params.h`
pub const PRE_BLOSSOM_HALVING_INTERVAL: Height = 840_000;

/// Transcription from `zcash/src/consensus/params.h`
pub const POST_BLOSSOM_HALVING_INTERVAL: Height =
    PRE_BLOSSOM_HALVING_INTERVAL * BLOSSOM_POW_TARGET_SPACING_RATIO;

/// Transcription from `zcash/src/consensus/params.h`
pub const SUBSIDY_SLOW_START_SHIFT: Height = SUBSIDY_SLOW_START_INTERVAL / 2;

// Transcription of `zcash/src/main.cpp` `GetBlockSubsidy`
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
    if height >= BLOSSOM_ACTIVATION {
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
