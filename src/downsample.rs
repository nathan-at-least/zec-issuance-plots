use crate::idealtime::DateTime;

const ANGULAR_THRESHOLD: f64 = 0.0001;

/// Downsample data sets by removing points where Y coordinates vary little.

pub type Point<X> = (X, f32);

pub trait CoerceToF64: Clone + std::fmt::Debug {
    fn coerce_to_f64(&self) -> f64;
}

impl CoerceToF64 for DateTime {
    fn coerce_to_f64(&self) -> f64 {
        self.timestamp() as f64
    }
}

#[cfg(test)]
impl CoerceToF64 for usize {
    fn coerce_to_f64(&self) -> f64 {
        *self as f64
    }
}

pub fn downsample<I, X>(pts: I) -> impl Iterator<Item = Point<X>>
where
    I: Iterator<Item = Point<X>>,
    X: CoerceToF64,
{
    Downsampler { pts, seg: None }
}

struct Downsampler<I, X> {
    pts: I,
    seg: Option<LineSegment<X>>,
}

#[derive(Debug)]
enum LineSegment<X> {
    Singleton(Point<X>),
    Span {
        start: Point<X>,
        end: Point<X>,
        latest: Point<X>,
    },
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
        optpt
    }
}

impl<X> LineSegment<X>
where
    X: CoerceToF64,
{
    fn transition(self, pt: Point<X>) -> (Self, Option<Point<X>>) {
        use LineSegment::*;

        match self {
            Singleton(first) => (
                Span {
                    start: first,
                    end: pt.clone(),
                    latest: pt,
                },
                None,
            ),
            Span { start, end, latest } => {
                let current = angle(&start, &end);
                let new = angle(&start, &pt);
                if (current - new).abs() >= ANGULAR_THRESHOLD {
                    // Set a new line and emit the previous line start pt:
                    (
                        Span {
                            start: latest,
                            end: pt.clone(),
                            latest: pt,
                        },
                        Some(start),
                    )
                } else {
                    // Keep the same line:
                    (
                        Span {
                            start,
                            end,
                            latest: pt,
                        },
                        None,
                    )
                }
            }
        }
    }

    fn wind_down(self) -> (Option<Self>, Option<Point<X>>) {
        use LineSegment::*;

        match self {
            Singleton(first) => (None, Some(first)),
            Span {
                start,
                end: _,
                latest,
            } => (Some(Singleton(latest)), Some(start)),
        }
    }
}

fn angle<X>(start: &Point<X>, end: &Point<X>) -> f64
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

    ((ye - ys) / (xe - xs)).atan()
}

#[cfg(test)]
mod tests;
