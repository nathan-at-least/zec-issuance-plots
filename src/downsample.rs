use crate::idealtime::DateTime;

const THRESHOLD: f64 = 0.0001;

/// Downsample data sets by removing points where Y coordinates vary little.

pub type Point<X> = (X, f32);

pub trait CoerceToF64: std::fmt::Debug {
    fn coerce_to_f64(&self) -> f64;
}

impl CoerceToF64 for DateTime {
    fn coerce_to_f64(&self) -> f64 {
        self.timestamp() as f64
    }
}

pub fn downsample<I, X>(pts: I) -> impl Iterator<Item = Point<X>>
where
    I: Iterator<Item = Point<X>>,
    X: CoerceToF64,
{
    Downsampler {
        pts: pts,
        seg: None,
    }
}

struct Downsampler<I, X> {
    pts: I,
    seg: Option<LineSegment<X>>,
}

#[derive(Debug)]
enum LineSegment<X> {
    Singleton(Point<X>),
    Span(Point<X>, Point<X>),
}

impl<I, X> Iterator for Downsampler<I, X>
where
    I: Iterator<Item = Point<X>>,
    X: CoerceToF64,
{
    type Item = Point<X>;

    fn next(&mut self) -> Option<Self::Item> {
        for pt in self.pts.by_ref() {
            if let Some(seg) = self.seg.take() {
                let (newseg, optpt) = seg.transition(pt);
                self.seg = Some(newseg);
                if optpt.is_some() {
                    return optpt;
                }
            } else {
                self.seg = Some(LineSegment::Singleton(pt));
            }
        }

        let (newseg, optpt) = self
            .seg
            .take()
            .map(|seg| seg.wind_down())
            .unwrap_or((None, None));
        self.seg = newseg;
        return optpt;
    }
}

impl<X> LineSegment<X>
where
    X: CoerceToF64,
{
    fn transition(self, pt: Point<X>) -> (Self, Option<Point<X>>) {
        use LineSegment::*;

        match self {
            Singleton(first) => (Span(first, pt), None),
            Span(start, end) => {
                let curslope = slope(&start, &end);
                let newslope = slope(&start, &pt);
                if (curslope - newslope).abs() / curslope >= THRESHOLD {
                    (Span(end, pt), Some(start))
                } else {
                    (Span(start, pt), None)
                }
            }
        }
    }

    fn wind_down(self) -> (Option<Self>, Option<Point<X>>) {
        use LineSegment::*;

        match self {
            Singleton(first) => (None, Some(first)),
            Span(start, end) => (Some(Singleton(end)), Some(start)),
        }
    }
}

fn slope<X>(start: &Point<X>, end: &Point<X>) -> f64
where
    X: CoerceToF64,
{
    fn f32pt<X>(&(ref rawx, ref y): &Point<X>) -> (f64, f64)
    where
        X: CoerceToF64,
    {
        (rawx.coerce_to_f64(), *y as f64)
    }

    let (xs, ys) = f32pt(start);
    let (xe, ye) = f32pt(end);
    assert!(xs != xe);

    (ye - ys) / (xe - xs)
}

#[cfg(test)]
mod tests;
