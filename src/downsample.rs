const THRESHOLD: f32 = 0.01;

/// Downsample data sets by removing points where Y coordinates vary little.
pub fn downsample<I, X>(pts: I) -> impl Iterator<Item = (X, f32)>
where
    I: Iterator<Item = (X, f32)>,
{
    Downsampler { pts, state: None }
}

struct Downsampler<I, X>
where
    I: Iterator<Item = (X, f32)>,
{
    pts: I,
    state: Option<(f32, (X, f32))>,
}

impl<I, X> Iterator for Downsampler<I, X>
where
    I: Iterator<Item = (X, f32)>,
{
    type Item = (X, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(pt) = self.pts.next() {
            let y = pt.1;

            if let Some((latch, prevpt)) = self.state.take() {
                if (latch - y) / latch >= THRESHOLD {
                    self.state = Some((y, pt));
                    Some(prevpt)
                } else {
                    self.state = Some((latch, pt));
                    None
                }
            } else {
                self.state = Some((y, pt));
                None
            }
        } else {
            self.state.take().map(|(_, lastpt)| lastpt)
        }
    }
}

#[cfg(test)]
mod tests;
