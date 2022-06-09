use crate::idealtime::{self, DateTime};
use crate::units::Height;
use plotters::coord::types::IntoMonthly;
use plotters::prelude::*;
use std::ops::Range;

#[derive(Debug)]
pub struct LinePlot<I> {
    pub file_stem: &'static str,
    pub caption: &'static str,
    pub x_range: Range<Height>,
    pub y_range: Range<f32>,
    pub points: I,
}

impl<I> LinePlot<I>
where
    I: Iterator<Item = (DateTime, f32)>,
{
    pub fn plot(self) -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("plots/{}.png", self.file_stem);
        let root = BitMapBackend::new(&path, (960, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        let x_range_time = idealtime::range(self.x_range.start, self.x_range.end);

        let mut chart = ChartBuilder::on(&root)
            .caption(self.caption, ("sans-serif", 20).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_range_time.monthly(), self.y_range)?;

        chart.configure_mesh().disable_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(self.points, &RED))?
            .label("FIXME")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        Ok(())
    }
}
