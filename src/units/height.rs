use derive_more::{From, Into};
use num_derive::NumOps;

#[derive(Debug, NumOps, From, Into)]
pub struct Height(usize);

impl Height {
    pub fn is_below_slow_start_shift(&self) -> bool {
        self < &Self::subsidy_slow_start_shift()
    }

    pub fn is_below_slow_start_interval(&self) -> bool {
        self < &Self::subsidy_slow_start_interval()
    }

    pub fn is_blossom_active(&self) -> bool {
        self >= &Self::blossom_activation_height()
    }

    pub fn subsidy_slow_start_interval() -> Height {
        // Transcription of `zcash/src/chainparams.cpp` `CMainParams`
        Height(20_000)
    }

    // Transcription of `zcash/src/chainparams.cpp` `CMainParams`
    pub fn blossom_activation_height() -> Height {
        Height(653_600)
    }

    // Transcription of `zcash/src/consensus/params.h`
    pub fn subsidy_slow_start_shift() -> Height {
        // 2 derived from manual arithmetic evaluation from `Params`
        Height::subsidy_slow_start_interval() / 2
    }

    // Transcription of `zcash/src/consensus/params.h`
    pub fn halving_interval_pre_blossom() -> Height {
        Height::from(840_000)
    }

    // Transcription of `zcash/src/consensus/params.h`
    // Note: This isn't a height, but it's most convenient here.
    pub fn blossom_pow_target_spacing_ratio() -> usize {
        2
    }

    // Transcription of `zcash/src/consensus/params.h`
    pub fn halving_interval_post_blossom() -> Height {
        Height::halving_interval_pre_blossom() * Height::blossom_pow_target_spacing_ratio()
    }
}
