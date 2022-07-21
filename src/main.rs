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
    std::fs::create_dir_all("plots")?;

    let max_height = {
        let h = halving_height(10);
        h + (h / 10)
    };

    // This is an arbitrary activation height for illustration:
    let activation_height = halving_height(2) - (POST_BLOSSOM_HALVING_INTERVAL / 5);
    let tes = TailEmissionSubsidy::subsidy_from_activation_height(activation_height);

    LinePlot {
        file_stem: "issuance",
        caption: "ZEC Issuance per 10m Interval (NU5)",
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
