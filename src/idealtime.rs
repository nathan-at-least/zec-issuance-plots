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
        blossom_activation, post_blossom_pow_target_spacing, pre_blossom_pow_target_spacing,
    };
    use std::cmp::{max, min};

    let pre_blossom_blocks = min(h, blossom_activation());
    let post_blossom_blocks = max(h, blossom_activation()) - blossom_activation();

    let pre_blossom_seconds = pre_blossom_blocks * pre_blossom_pow_target_spacing();
    let post_blossom_seconds = post_blossom_blocks * post_blossom_pow_target_spacing();
    let seconds_since_genesis = pre_blossom_seconds + post_blossom_seconds;

    genesis() + seconds_since_genesis
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
