//! The functions named in this module read best at call sites that use the mod name in the path,
//! ie: `idealtime::at(h)`.

use crate::units::Height;
use chrono::offset::Utc;
use chrono::Duration;
use std::str::FromStr;

const GENESIS_TIME_TEXT_ZCASH: &str = "2016-10-28 07:56:00 UTC";
const GENESIS_TIME_TEXT_BITCOIN: &str = "2009-01-03 18:15:05 UTC";

pub type DateTime = chrono::DateTime<Utc>;

pub fn bitcoin_block_target() -> Duration {
    Duration::minutes(10)
}

#[derive(Clone, Copy)]
pub enum Chain {
    Bitcoin,
    Zcash,
}

#[derive(Clone, Copy)]
pub struct TimeModel {
    chain: Chain,
    genesis: DateTime,
}

impl TimeModel {
    pub fn new(chain: Chain) -> Self {
        let gentext = match chain {
            Chain::Bitcoin => GENESIS_TIME_TEXT_BITCOIN,
            Chain::Zcash => GENESIS_TIME_TEXT_ZCASH,
        };
        let genesis = DateTime::from_str(gentext).unwrap();

        TimeModel { chain, genesis }
    }

    pub fn at(&self, h: Height) -> DateTime {
        match self.chain {
            Chain::Bitcoin => self.genesis + (bitcoin_block_target() * h.try_into().unwrap()),
            Chain::Zcash => zcash_time_at(self.genesis, h),
        }
    }
}

fn zcash_time_at(genesis: DateTime, h: Height) -> DateTime {
    use crate::consts::{
        BLOSSOM_ACTIVATION, POST_BLOSSOM_POW_TARGET_SPACING, PRE_BLOSSOM_POW_TARGET_SPACING,
    };
    use std::cmp::{max, min};

    let pre_blossom_blocks = min(h, BLOSSOM_ACTIVATION);
    let post_blossom_blocks = max(h, BLOSSOM_ACTIVATION) - BLOSSOM_ACTIVATION;

    let pre_blossom_seconds = pre_blossom_blocks * PRE_BLOSSOM_POW_TARGET_SPACING;
    let post_blossom_seconds = post_blossom_blocks * POST_BLOSSOM_POW_TARGET_SPACING;
    let seconds_since_genesis = pre_blossom_seconds + post_blossom_seconds;

    genesis + Duration::seconds(seconds_since_genesis as i64)
}
