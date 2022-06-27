use crate::consts::COIN;
use crate::idealtime::{self, DateTime};
use crate::timebuckets::TimeBucketIter;
use crate::units::{Height, Zat};
use plotters::coord::types::IntoMonthly;
use plotters::prelude::*;
use std::ops::Range;

const PALETTE: &[RGBColor] = &[BLUE, GREEN];

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
        let path = format!("plots/{}.png", self.file_stem);
        let root = BitMapBackend::new(&path, (960, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let datasets: Vec<DataSet<DateTime, f32>> = self
            .datasets
            .into_iter()
            .map(|dset| DataSet {
                name: dset.name,
                points: TimeBucketIter::new(
                    dset.points
                        .into_iter()
                        .map(|(h, zat)| (idealtime::at(h), zat2zec(zat))),
                    idealtime::bitcoin_block_target(),
                )
                .collect(),
            })
            .collect();

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
            .caption(self.caption, ("sans-serif", 20).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d((time_min..time_max).monthly(), 0f32..zec_max)?;

        chart.configure_mesh().disable_mesh().draw()?;

        for (ix, dset) in datasets.into_iter().enumerate() {
            let color = PALETTE[ix % PALETTE.len()];
            chart
                .draw_series(LineSeries::new(dset.points.into_iter(), color))?
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
