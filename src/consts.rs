use crate::units::{Height, RatioU64, Seconds, Zat};
use std::str::FromStr;

/// Transcription from `zcash/src/main.cpp`
pub fn start_subsidy() -> Zec {
    Zec::from_str("12.5").unwrap()
}

/// Transcription from `zcash/src/chainparams.cpp`
pub fn subsidy_slow_start_interval() -> Height {
    Height::from(20_000)
}

/// Transcription from `zcash/src/chainparams.cpp`
pub fn blossom_activation() -> Height {
    Height::from(653_600)
}

/// Transcription from `zcash/src/consensus/params.h`
pub fn pre_blossom_pow_target_spacing() -> Seconds {
    Seconds::from(150)
}

/// Transcription from `zcash/src/consensus/params.h`
pub fn post_blossom_pow_target_spacing() -> Seconds {
    Seconds::from(75)
}

/// Transcription from `zcash/src/consensus/params.h`
pub fn blossom_pow_target_spacing_ratio() -> RatioU64 {
    pre_blossom_pow_target_spacing() / post_blossom_pow_target_spacing()
}

/// Transcription from `zcash/src/consensus/params.h`
pub fn pre_blossom_halving_interval() -> Height {
    Height::from(840_000)
}

/// Transcription from `zcash/src/consensus/params.h`
pub fn post_blossom_halving_interval() -> Height {
    pre_blossom_halving_interval() * blossom_pow_target_spacing_ratio()
}

/// Transcription from `zcash/src/consensus/params.h`
pub fn subsidy_slow_start_shift() -> Height {
    subsidy_slow_start_interval() / 2
}
