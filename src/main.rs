mod consts;
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
    std::fs::create_dir_all("plots")?;

    let max_height = {
        let h = halving_height(3);
        h + (h / 10)
    };

    LinePlot {
        file_stem: "issuance",
        caption: "ZEC Issuance per 10m Interval (NU5)",
        datasets: vec![DataSet::build("NU5", 0..max_height, |h| {
            NU5.block_subsidy(h)
        })],
    }
    .plot()?;

    Ok(())
}
