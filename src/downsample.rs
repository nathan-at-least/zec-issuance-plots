pub fn downsample<I, X, Y>(pts: I) -> impl Iterator<Item = (X, Y)>
where
    I: Iterator<Item = (X, Y)>,
{
    pts.step_by(2000)
}
