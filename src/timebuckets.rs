use crate::idealtime::DateTime;
use chrono::Duration;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct TimeBucketIter<I, Y>
where
    I: Iterator<Item = (DateTime, Y)>,
    Y: AddAssign + Copy,
{
    it: I,
    windowsize: Duration,
    bucket: Option<Bucket<Y>>,
}

#[derive(Debug)]
pub struct Bucket<Y>
where
    Y: AddAssign + Copy,
{
    start: DateTime,
    sum: Y,
}

impl<I, Y> TimeBucketIter<I, Y>
where
    I: Iterator<Item = (DateTime, Y)>,
    Y: AddAssign + Copy,
{
    pub fn new(it: I, windowsize: Duration) -> Self {
        TimeBucketIter {
            it,
            windowsize,
            bucket: None,
        }
    }
}

impl<I, Y> Iterator for TimeBucketIter<I, Y>
where
    I: Iterator<Item = (DateTime, Y)>,
    Y: AddAssign + Copy,
{
    type Item = (DateTime, Y);

    fn next(&mut self) -> Option<Self::Item> {
        for (t, y) in self.it.by_ref() {
            if let Some(bucket) = self.bucket.as_mut() {
                if t - bucket.start >= self.windowsize {
                    let next = (bucket.start, bucket.sum);
                    self.bucket = Some(Bucket { start: t, sum: y });
                    return Some(next);
                } else {
                    bucket.sum += y;
                }
            } else {
                self.bucket = Some(Bucket { start: t, sum: y });
            }
        }
        if let Some(bucket) = self.bucket.as_ref() {
            let last = (bucket.start, bucket.sum);
            self.bucket = None;
            Some(last)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests;
