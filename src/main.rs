mod consts;
mod halving;
mod idealtime;
mod plot;
mod subsidy;
mod timebuckets;
mod units;

use self::plot::plot;
use self::units::Zat;
use crate::consts::{COIN, START_SUBSIDY};
use crate::halving::halving_height;
use crate::subsidy::Subsidy::NU5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_height = {
        let h = halving_height(3);
        h + (h / 10)
    };

    let raw_points = (0..max_height).map(|h| (idealtime::at(h), zat2zec(NU5.block_subsidy(h))));

    plot(
        idealtime::range(0, max_height),
        0f32..zat2zec(START_SUBSIDY),
        timebuckets::TimeBucketIter::new(raw_points, chrono::Duration::minutes(10)),
    )?;
    Ok(())
}

fn zat2zec(zat: Zat) -> f32 {
    zat as f32 / COIN as f32
}
