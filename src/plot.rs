mod csv;

use crate::consts::COIN;
use crate::downsample::downsample;
use crate::idealtime::{bitcoin_block_target, Chain, DateTime, TimeModel};
use crate::timebuckets::TimeBucketIter;
use crate::units::{Height, Zat};
use plotters::coord::types::IntoMonthly;
use plotters::prelude::*;
use std::ops::Range;

const PLOT_SIZE: (u32, u32) = (1920, 960);

const PALETTE: &[RGBColor] = &[
    RGBColor(84, 85, 108),
    RGBColor(17, 93, 118),
    RGBColor(211, 182, 41),
    RGBColor(28, 82, 83),
    RGBColor(130, 80, 82),
];

#[derive(Debug)]
pub struct LinePlot {
    pub file_stem: &'static str,
    pub caption: &'static str,
    pub datasets: Vec<DataSet<Height, Zat>>,
}

#[derive(Debug)]
pub struct DataSet<X, Y> {
    name: &'static str,
    points: Vec<(X, Y)>,
}

impl LinePlot {
    pub fn plot(self) -> Result<(), Box<dyn std::error::Error>> {
        let zctime = TimeModel::new(Chain::Zcash);

        let path = format!("plots/{}.png", self.file_stem);
        println!("Generating plot {} in {:?}", self.file_stem, &path);
        let root = BitMapBackend::new(&path, PLOT_SIZE).into_drawing_area();
        root.fill(&WHITE)?;

        let datasets: Vec<DataSet<DateTime, f32>> = self
            .datasets
            .into_iter()
            .map(|dset| DataSet {
                name: dset.name,
                points: downsample(TimeBucketIter::new(
                    dset.points
                        .into_iter()
                        .map(|(h, zat)| (zctime.at(h), zat2zec(zat))),
                    bitcoin_block_target(),
                ))
                .collect(),
            })
            .collect();

        csv::write(self.file_stem, &datasets)?;

        let time_min = *datasets
            .iter()
            .map(|dset| dset.points.iter().map(|(t, _)| t).min().unwrap())
            .min()
            .unwrap();

        let time_max = *datasets
            .iter()
            .map(|dset| dset.points.iter().map(|(t, _)| t).max().unwrap())
            .max()
            .unwrap();

        let zec_max = datasets
            .iter()
            .map(|dset| {
                dset.points
                    .iter()
                    .map(|(_, z)| z)
                    .fold(0f32, |a, b| max_f32(a, *b))
            })
            .fold(0f32, max_f32)
            * 1.1;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.caption, ("sans-serif", 40).into_font())
            .margin(5)
            .x_label_area_size(60)
            .y_label_area_size(60)
            .build_cartesian_2d((time_min..time_max).monthly(), 0f32..zec_max)?;

        chart
            .configure_mesh()
            .disable_mesh()
            .label_style(("sans-serif", 25))
            .draw()?;

        for (ix, dset) in datasets.into_iter().enumerate() {
            let color = PALETTE[ix % PALETTE.len()];
            let points: Vec<_> = dset.points.into_iter().collect();
            chart.draw_series(
                points
                    .clone()
                    .into_iter()
                    .map(|pt| Circle::new(pt, 5, color)),
            )?;
            chart
                .draw_series(LineSeries::new(points, color))?
                .label(dset.name)
                .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(())
    }
}

impl DataSet<Height, Zat> {
    pub fn build<F>(name: &'static str, xrange: Range<Height>, f: F) -> Self
    where
        F: Fn(Height) -> Zat,
    {
        println!("Building data set {}", name);
        DataSet {
            name,
            points: xrange.map(|x| (x, f(x))).collect(),
        }
    }
}

fn zat2zec(zat: Zat) -> f32 {
    zat as f32 / COIN as f32
}

fn max_f32(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}
