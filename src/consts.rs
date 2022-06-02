use crate::units::{Height, Zat};

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
