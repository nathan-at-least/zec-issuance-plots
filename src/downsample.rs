pub fn downsample<I, X, Y>(pts: I) -> impl Iterator<Item = (X, Y)>
where
    I: Iterator<Item = (X, Y)>,
{
    Downsampler {
        it: pts.fuse(),
        firstyielded: false,
        stride: 2000,
    }
}

/// An iterator than yields the first item, then skips `stride` for every item after with the
/// exception that it always yields the final item of the underlying iterator.
struct Downsampler<I> {
    it: std::iter::Fuse<I>,
    firstyielded: bool,
    stride: usize,
}

impl<I, X> Iterator for Downsampler<I>
where
    I: Iterator<Item = X>,
{
    type Item = X;

    fn next(&mut self) -> Option<X> {
        if !self.firstyielded {
            self.firstyielded = true;
            self.it.next()
        } else {
            let mut lastseen = None;

            for _ in 0..self.stride {
                let optx = self.it.next();
                if optx.is_none() {
                    return lastseen;
                }
                lastseen = optx;
            }

            lastseen
        }
    }
}

#[cfg(test)]
mod tests;
