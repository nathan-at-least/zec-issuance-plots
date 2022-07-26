const THRESHOLD: f32 = 0.01;

/// Downsample data sets by removing points where Y coordinates vary little.

pub type Point<X> = (X, f32);

pub fn downsample<I, X>(pts: I) -> impl Iterator<Item = Point<X>>
where
    I: Iterator<Item = Point<X>>,
    X: std::fmt::Debug,
{
    Downsampler {
        pts: Some(pts),
        latch: None,
        yieldlatch: None,
    }
}

struct Downsampler<I, X> {
    pts: Option<I>,
    latch: Option<Latch<X>>,
    yieldlatch: Option<Latch<X>>,
}

#[derive(Debug)]
enum Latch<X> {
    Singleton(Point<X>),
    Span(Point<X>, Point<X>),
}

impl<I, X> Iterator for Downsampler<I, X>
where
    I: Iterator<Item = Point<X>>,
    X: std::fmt::Debug,
{
    type Item = Point<X>;

    fn next(&mut self) -> Option<Self::Item> {
        use Latch::*;

        if let Some(yl) = self.yieldlatch.take() {
            match yl {
                Singleton(pt) => Some(pt),
                Span(a, b) => {
                    self.yieldlatch = Some(Singleton(b));
                    Some(a)
                }
            }
        } else if let Some(pts) = self.pts.as_mut() {
            for pt in pts {
                if let Some(latch) = self.latch.take() {
                    if latch.past_threshold(pt.1) {
                        self.yieldlatch = Some(latch);
                        self.latch = Some(Singleton(pt));
                        return self.next();
                    } else {
                        self.latch = Some(latch.update(pt))
                    }
                } else {
                    self.latch = Some(Singleton(pt));
                }
            }

            self.pts = None;
            self.yieldlatch = self.latch.take();
            self.next()
        } else {
            None
        }
    }
}

impl<X> Latch<X> {
    fn past_threshold(&self, y: f32) -> bool {
        let latchval = self.latch_val();
        let delta = (latchval - y).abs();
        let thresh = if latchval < THRESHOLD / 1e6 {
            // Avoid divide by 0:
            delta
        } else {
            delta / latchval
        };

        thresh >= THRESHOLD
    }

    fn latch_val(&self) -> f32 {
        use Latch::*;

        match self {
            Singleton((_, y)) => *y,
            Span((_, y), _) => *y,
        }
    }

    fn update(self, pt: Point<X>) -> Self {
        use Latch::*;

        match self {
            Singleton(a) => Span(a, pt),
            Span(a, _) => Span(a, pt),
        }
    }
}

#[cfg(test)]
mod tests;
