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
use crate::subsidy::Subsidy::{Btc, NU5};
use crate::units::{Height, Zat};
use std::ops::Range;

const PLOTS_DIR: &str = "plots";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    plotsdir::refresh()?;

    let max_supply = 21_000_000f64;
    let zec_max_height = {
        let h = halving_height(5);
        h + (h / 10)
    };

    let zectime = TimeModel::new(Chain::Zcash);
    let btctime = TimeModel::new(Chain::Bitcoin);

    let endtime = zectime.at(zec_max_height);
    let btc_max_height = ((endtime - btctime.at(0)).num_minutes() as u64) / 10;

    dbg!(
        zectime.at(0),
        zectime.at(zec_max_height),
        zec_max_height,
        btctime.at(0),
        btctime.at(btc_max_height),
        btc_max_height
    );

    LinePlot {
        file_stem: "issuance-nu5",
        caption: "ZEC Issuance per 10m Interval (as of NU5 protocol)",
        datasets: vec![gen_issuance_dataset("NU5", 0..zec_max_height, |h| {
            NU5.block_subsidy(h)
        })],
        points: false,
    }
    .plot()?;

    LinePlot {
        file_stem: "supply-btc-vs-nu5",
        caption: "Supplies of BTC & ZEC (as of NU5 protocol)",
        datasets: vec![
            DataSet::new("supply cap", {
                vec![
                    (btctime.at(0), max_supply),
                    (btctime.at(btc_max_height), max_supply),
                ]
            }),
            gen_supply_dataset(zectime, "ZEC (NU5)", 0..zec_max_height, |h| {
                NU5.block_subsidy(h)
            }),
            gen_supply_dataset(btctime, "BTC", 0..btc_max_height, |h| Btc.block_subsidy(h)),
        ],
        points: false,
    }
    .plot()?;

    Ok(())
}

fn gen_issuance_dataset<F>(
    name: &'static str,
    heights: Range<Height>,
    f: F,
) -> DataSet<DateTime, f64>
where
    F: Fn(Height) -> Zat,
{
    use crate::timebuckets::TimeBucketIter;

    let zctime = TimeModel::new(Chain::Zcash);

    println!("Building issuance dataset {}...", name);
    DataSet::new(
        name,
        TimeBucketIter::new(
            heights.map(move |h| (zctime.at(h), zat2zec(f(h)))),
            bitcoin_block_target(),
        )
        .collect(),
    )
}

fn gen_supply_dataset<F>(
    time: TimeModel,
    name: &'static str,
    heights: Range<Height>,
    f: F,
) -> DataSet<DateTime, f64>
where
    F: Fn(Height) -> Zat,
{
    println!("Building supply dataset {}...", name);
    DataSet::new(
        name,
        heights
            .map(move |h| (time.at(h), zat2zec(f(h))))
            .scan(0.0, |acc, (t, y)| {
                *acc += y;
                Some((t, *acc))
            })
            .collect(),
    )
}

fn zat2zec(zat: Zat) -> f64 {
    zat as f64 / COIN as f64
}
