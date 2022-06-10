mod consts;
mod halving;
mod idealtime;
mod plot;
mod subsidy;
mod timebuckets;
mod units;

use self::plot::LinePlot;
use crate::consts::start_subsidy;
use crate::halving::halving_height;
use crate::subsidy::Subsidy::NU5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("plots")?;

    println!(
        "MAX SUPPLY FOR {:?}: {:.8} ZEC",
        NU5,
        f32::from(NU5.max_supply().unwrap())
    );

    let max_height = {
        let h = halving_height(3);
        h + (h / 10)
    };

    let raw_points = (0..max_height).map(|h| (idealtime::at(h), f32::from(NU5.block_subsidy(h))));

    LinePlot {
        file_stem: "issuance",
        caption: "ZEC Issuance per 10m Interval (NU5)",
        x_range: idealtime::range(0, max_height),
        y_range: 0f32..(f32::from(4 * start_subsidy()) * 1.05),
        points: timebuckets::TimeBucketIter::new(raw_points, idealtime::bitcoin_block_target()),
    }
    .plot()?;

    Ok(())
}
