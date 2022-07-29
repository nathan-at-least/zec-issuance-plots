mod consts;
mod downsample;
mod halving;
mod idealtime;
mod plot;
mod plotsdir;
mod subsidy;
mod timebuckets;
mod units;

use self::plot::{DataSet, LinePlot};
use crate::consts::COIN;
use crate::halving::halving_height;
use crate::idealtime::{bitcoin_block_target, Chain, DateTime, TimeModel};
use crate::subsidy::Subsidy::NU5;
use crate::units::{Height, Zat};
use std::ops::Range;

const PLOTS_DIR: &str = "plots";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    plotsdir::refresh()?;

    let max_height = {
        let h = halving_height(10);
        h + (h / 10)
    };

    let nu5_issuance = gen_height_dataset("NU5", 0..max_height, |h| NU5.block_subsidy(h));

    LinePlot {
        file_stem: "issuance-current",
        caption: "ZEC Issuance (current) per 10m Interval",
        datasets: vec![nu5_issuance.clone()],
    }
    .plot()?;

    Ok(())
}

fn gen_height_dataset<F>(name: &'static str, heights: Range<Height>, f: F) -> DataSet<DateTime, f32>
where
    F: Fn(Height) -> Zat,
{
    use crate::timebuckets::TimeBucketIter;

    let zctime = TimeModel::new(Chain::Zcash);

    println!("Building dataset {}...", name);
    DataSet::new(
        name,
        TimeBucketIter::new(
            heights.map(move |h| (zctime.at(h), zat2zec(f(h)))),
            bitcoin_block_target(),
        )
        .collect(),
    )
}

fn zat2zec(zat: Zat) -> f32 {
    zat as f32 / COIN as f32
}
