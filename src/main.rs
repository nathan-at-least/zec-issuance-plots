mod consts;
mod halving;
mod plot;
mod subsidy;
mod units;

use self::plot::plot;
use self::units::Zat;
use crate::consts::{COIN, START_SUBSIDY};
use crate::halving::halving_height;
use crate::subsidy::Subsidy::NU5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_height = {
        let hh2 = halving_height(2);
        hh2 + (hh2 / 10)
    };

    plot(
        0f32..(max_height as f32),
        0f32..zat2zec(START_SUBSIDY),
        (0..max_height).map(|h| (h as f32, zat2zec(NU5.block_subsidy(h)))),
    )?;
    Ok(())
}

fn zat2zec(zat: Zat) -> f32 {
    zat as f32 / COIN as f32
}
