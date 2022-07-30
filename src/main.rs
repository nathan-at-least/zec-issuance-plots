mod consts;
mod downsample;
mod halving;
mod idealtime;
mod plot;
mod subsidy;
mod timebuckets;
mod units;

use self::plot::{DataSet, LinePlot};
use crate::halving::halving_height;
use crate::subsidy::Subsidy::NU5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_height = {
        let h = halving_height(10);
        h + (h / 10)
    };

    std::fs::create_dir_all("plots")?;
    plot_issuance_current(max_height)?;
    Ok(())
}

fn plot_issuance_current(max_height: u64) -> Result<(), Box<dyn std::error::Error>> {
    LinePlot {
        file_stem: "issuance-current",
        caption: "ZEC Issuance (current) per 10m Interval",
        datasets: vec![DataSet::build("NU5", 0..max_height, |h| {
            NU5.block_subsidy(h)
        })],
    }
    .plot()?;

    Ok(())
}
