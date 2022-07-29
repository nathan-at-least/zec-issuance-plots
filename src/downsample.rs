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
        initial: Point<X>,
        start: Point<X>,
        end: Point<X>,
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
                    initial: first.clone(),
                    start: first,
                    end: pt,
                },
                None,
            ),
            Span {
                initial,
                start,
                end,
            } => {
                let current = angle(&start, &end);
                let new = angle(&end, &pt);
                if (current - new).abs() >= ANGULAR_THRESHOLD {
                    // Set a new line and emit the previous initial:
                    (
                        Span {
                            initial: end.clone(),
                            start: end,
                            end: pt,
                        },
                        Some(initial),
                    )
                } else {
                    // Adjust the line w/ same initial:
                    (
                        Span {
                            initial,
                            start: end,
                            end: pt,
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
                initial,
                start: _,
                end,
            } => (Some(Singleton(end)), Some(initial)),
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
