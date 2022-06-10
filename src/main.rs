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

    println!(
        "MAX SUPPLY FOR {:?}: {:.8} ZAT",
        NU5,
        NU5.max_supply().unwrap()
    );

    let max_height = {
        let h = halving_height(3);
        h + (h / 10)
    };

    LinePlot {
        file_stem: "issuance",
        caption: "ZEC Issuance per 10m Interval (NU5)",
        datasets: vec![DataSet {
            name: "NU5",
            points: (0..max_height).map(|h| (h, NU5.block_subsidy(h))).collect(),
        }],
    }
    .plot()?;

    Ok(())
}
