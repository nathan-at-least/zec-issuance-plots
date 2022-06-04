//! The functions named in this module read best at call sites that use the mod name in the path,
//! ie: `idealtime::at(h)`.

use crate::units::Height;
use chrono::offset::Utc;
use chrono::Duration;
use std::ops::Range;

pub type DateTime = chrono::DateTime<Utc>;

pub fn bitcoin_block_target() -> Duration {
    Duration::minutes(10)
}

const GENESIS_TIME_TEXT: &str = "2016-10-28 07:56:00 UTC";

pub fn at(h: Height) -> DateTime {
    use crate::consts::{
        BLOSSOM_ACTIVATION, POST_BLOSSOM_POW_TARGET_SPACING, PRE_BLOSSOM_POW_TARGET_SPACING,
    };
    use std::cmp::{max, min};

    let pre_blossom_blocks = min(h, BLOSSOM_ACTIVATION);
    let post_blossom_blocks = max(h, BLOSSOM_ACTIVATION) - BLOSSOM_ACTIVATION;

    let pre_blossom_seconds = pre_blossom_blocks * PRE_BLOSSOM_POW_TARGET_SPACING;
    let post_blossom_seconds = post_blossom_blocks * POST_BLOSSOM_POW_TARGET_SPACING;
    let seconds_since_genesis = pre_blossom_seconds + post_blossom_seconds;

    genesis() + Duration::seconds(seconds_since_genesis as i64)
}

pub fn range(start: Height, end: Height) -> Range<DateTime> {
    at(start)..at(end)
}

fn genesis() -> DateTime {
    use once_cell::sync::OnceCell;
    use std::str::FromStr;

    static CELL: OnceCell<DateTime> = OnceCell::new();

    *CELL.get_or_init(|| DateTime::from_str(GENESIS_TIME_TEXT).unwrap())
}
