use crate::idealtime::DateTime;
use plotters::prelude::*;
use std::ops::Range;

pub fn plot<I>(
    xrange: Range<DateTime>,
    yrange: Range<f32>,
    points: I,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: Iterator<Item = (DateTime, f32)>,
{
    let root = BitMapBackend::new("target/plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("FIXME", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(xrange, yrange)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(points, &RED))?
        .label("FIXME")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
