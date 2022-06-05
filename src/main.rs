mod consts;
mod halving;
mod idealtime;
mod plot;
mod subsidy;
mod timebuckets;
mod units;

use self::plot::LinePlot;
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

    LinePlot {
        xrange: idealtime::range(0, max_height),
        yrange: 0f32..zat2zec(4 * START_SUBSIDY),
        points: timebuckets::TimeBucketIter::new(raw_points, idealtime::bitcoin_block_target()),
    }
    .plot()?;

    Ok(())
}

fn zat2zec(zat: Zat) -> f32 {
    zat as f32 / COIN as f32
}
