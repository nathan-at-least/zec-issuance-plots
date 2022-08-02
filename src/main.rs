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
use crate::consts::{COIN, POST_BLOSSOM_HALVING_INTERVAL};
use crate::halving::halving_height;
use crate::idealtime::{bitcoin_block_target, Chain, DateTime, TimeModel};
use crate::subsidy::Subsidy::{self, Btc, PosterityFund, NU5};
use crate::units::Zat;

const PLOTS_DIR: &str = "plots";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    plotsdir::refresh()?;

    let max_supply = 21_000_000f64;
    let zec_max_height = {
        let h = halving_height(5);
        h + (h / 10)
    };
    let (zpf_activation_height_a, zpf_activation_height_b) = {
        let b = POST_BLOSSOM_HALVING_INTERVAL;
        let h = halving_height(2);
        (h - (b / 5), h - (b / 7))
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

    if false {
        LinePlot {
            file_stem: "issuance-nu5",
            caption: "ZEC Issuance per 10m Interval (as of NU5 protocol)",
            datasets: vec![gen_issuance_dataset(NU5, zec_max_height)],
            points: false,
        }
        .plot()?;

        LinePlot {
            file_stem: "supply-btc-vs-nu5",
            caption: "Supplies of BTC & ZEC (as of NU5 protocol)",
            datasets: vec![
                DataSet::new("supply cap".to_string(), {
                    vec![
                        (btctime.at(0), max_supply),
                        (btctime.at(btc_max_height), max_supply),
                    ]
                }),
                gen_supply_dataset(zectime, NU5, zec_max_height),
                gen_supply_dataset(btctime, Btc, btc_max_height),
            ],
            points: false,
        }
        .plot()?;
    }

    LinePlot {
        file_stem: "issuance-zpf",
        caption: "ZPF Issuance per 10m Interval",
        datasets: vec![
            gen_issuance_dataset(NU5, zec_max_height),
            gen_issuance_dataset(PosterityFund(zpf_activation_height_a), zec_max_height),
            gen_issuance_dataset(PosterityFund(zpf_activation_height_b), zec_max_height),
        ],
        points: false,
    }
    .plot()?;
    Ok(())
}

fn gen_issuance_dataset(subsidy: Subsidy, max_height: u64) -> DataSet<DateTime, f64> {
    use crate::timebuckets::TimeBucketIter;

    let name = subsidy.legend();
    let zctime = TimeModel::new(Chain::Zcash);

    println!("Building issuance dataset {}...", name);
    DataSet::new(
        name,
        TimeBucketIter::new(
            subsidy
                .into_iter()
                .take(usize::try_from(max_height).unwrap())
                .map(|(h, zat, _)| (zctime.at(h), zat2zec(zat))),
            bitcoin_block_target(),
        )
        .collect(),
    )
}

fn gen_supply_dataset(
    time: TimeModel,
    subsidy: Subsidy,
    max_height: u64,
) -> DataSet<DateTime, f64> {
    let name = subsidy.legend();
    println!("Building supply dataset {}...", name);
    DataSet::new(
        name,
        subsidy
            .into_iter()
            .take(usize::try_from(max_height).unwrap())
            .map(move |(h, _, supply)| (time.at(h), zat2zec(supply)))
            .collect(),
    )
}

fn zat2zec(zat: Zat) -> f64 {
    zat as f64 / COIN as f64
}
