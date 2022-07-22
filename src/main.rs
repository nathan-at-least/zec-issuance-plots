mod consts;
mod halving;
mod idealtime;
mod plot;
mod subsidy;
mod timebuckets;
mod units;

use self::plot::{DataSet, LinePlot};
use crate::consts::POST_BLOSSOM_HALVING_INTERVAL;
use crate::halving::halving_height;
use crate::subsidy::Subsidy::NU5;
use crate::subsidy::TailEmissionSubsidy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let max_height = {
        let h = halving_height(10);
        h + (h / 10)
    };

    std::fs::create_dir_all("plots")?;
    plot_issuance_current(max_height)?;
    plot_issuance_sketches(max_height)?;
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

fn plot_issuance_sketches(max_height: u64) -> Result<(), Box<dyn std::error::Error>> {
    // This is an arbitrary activation height for illustration:
    let activation_height = halving_height(2) - (POST_BLOSSOM_HALVING_INTERVAL / 5);
    let tes = TailEmissionSubsidy::subsidy_from_activation_height(activation_height);

    LinePlot {
        file_stem: "issuance-sketches",
        caption: "ZEC Issuance Sketches per 10m Interval",
        datasets: vec![
            DataSet::build("NU5", 0..max_height, |h| NU5.block_subsidy(h)),
            DataSet::build("2% Tail Emission", activation_height..max_height, |h| {
                tes.block_subsidy(h)
            }),
        ],
    }
    .plot()?;

    Ok(())
}
