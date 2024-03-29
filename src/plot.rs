mod csv;

use crate::downsample::downsample;
use crate::idealtime::DateTime;
use crate::PLOTS_DIR;
use plotters::coord::types::IntoMonthly;
use plotters::prelude::*;

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
    pub datasets: Vec<DataSet<DateTime, f64>>,
    pub points: bool,
}

#[derive(Clone, Debug)]
pub struct DataSet<X, Y> {
    name: String,
    points: Vec<(X, Y)>,
}

impl LinePlot {
    pub fn plot(self) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("{}/{}.png", PLOTS_DIR, self.file_stem);
        println!("Generating plot {} in {:?}", self.file_stem, &path);
        let root = BitMapBackend::new(&path, PLOT_SIZE).into_drawing_area();
        root.fill(&WHITE)?;

        let datasets: Vec<DataSet<DateTime, f64>> = self
            .datasets
            .into_iter()
            .map(|dset| DataSet {
                name: dset.name,
                points: downsample(dset.points.into_iter()).collect(),
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
                    .fold(0f64, |a, b| max_f64(a, *b))
            })
            .fold(0f64, max_f64)
            * 1.1;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.caption, ("sans-serif", 40).into_font())
            .margin(5)
            .x_label_area_size(60)
            .y_label_area_size(140)
            .build_cartesian_2d((time_min..time_max).monthly(), 0f64..zec_max)?;

        chart
            .configure_mesh()
            .disable_mesh()
            .label_style(("sans-serif", 25))
            .draw()?;

        for (ix, dset) in datasets.into_iter().enumerate() {
            let color = PALETTE[ix % PALETTE.len()];

            if self.points {
                chart.draw_series(
                    dset.points
                        .into_iter()
                        .map(|pt| Circle::new(pt, 5.0, color)),
                )?;
            } else {
                chart
                    .draw_series(LineSeries::new(dset.points, color))?
                    .label(dset.name)
                    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
            }
        }

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(())
    }
}

impl<X, Y> DataSet<X, Y> {
    pub fn new(name: String, points: Vec<(X, Y)>) -> Self {
        DataSet { name, points }
    }
}

fn max_f64(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}
