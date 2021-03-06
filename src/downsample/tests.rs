use crate::downsample::downsample;
use test_case::test_case;

#[test_case([] => Vec::<f32>::new())]
#[test_case(
    [
        0.0,
    ] => vec![
        0.0,
    ]
)]
#[test_case(
    [
        0.0,
        0.0,
    ] => vec![
        0.0,
        0.0,
    ]
)]
#[test_case(
    [
        0.0,
        2e-8,
        0.0,
    ] => vec![
        0.0,
        0.0,
    ]
)]
#[test_case(
    [
        0.0,
        7.0,
        0.0,
    ] => vec![
        0.0,
        7.0,
        0.0,
    ]
)]
#[test_case(
    [
        0.0,
        7.0,
        7.00002,
        0.0,
    ] => vec![
        0.0,
        7.0,
        7.00002,
        0.0,
    ]
)]
#[test_case(
    [
        0.0,
        7.0,
        7.0 + 2e-7,
        7.0 + 3e-7,
        42.0,
    ] => vec![
        0.0,
        7.0,
        7.0 + 3e-7,
        42.0,
    ]
)]
fn test_downsample<const K: usize>(inputs: [f32; K]) -> Vec<f32> {
    dbg!(&inputs);
    downsample(inputs.into_iter().enumerate())
        .map(|(_, y)| y)
        .collect()
}
