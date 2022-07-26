use test_case::test_case;

#[test_case([] => Vec::<(usize, f32)>::new())]
#[test_case([(0, 0.0)] => vec![(0, 0.0)])]
fn downsample<const K: usize>(inputs: [(usize, f32); K]) -> Vec<(usize, f32)> {
    crate::downsample::downsample(inputs.into_iter()).collect()
}
