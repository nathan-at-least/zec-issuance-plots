use super::Downsampler;
use test_case::test_case;

#[test_case(0 => Vec::<usize>::new())]
#[test_case(1 => vec![0])]
#[test_case(2 => vec![0, 1])]
#[test_case(3 => vec![0, 2])]
#[test_case(5 => vec![0, 3, 4])]
#[test_case(6 => vec![0, 3, 5])]
#[test_case(7 => vec![0, 3, 6])]
#[test_case(8 => vec![0, 3, 6, 7])]
fn downsample(end: usize) -> Vec<usize> {
    Downsampler {
        it: (0..end).fuse(),
        firstyielded: false,
        stride: 3,
    }
    .collect()
}
