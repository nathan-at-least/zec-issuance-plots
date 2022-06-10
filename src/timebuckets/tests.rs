use crate::halving::halving_height;
use crate::idealtime;
use crate::subsidy::Subsidy::NU5;
use crate::timebuckets::TimeBucketIter;
use crate::units::Zat;
use test_case::test_case;

#[test_case(1)]
#[test_case(2)]
#[test_case(3)]
#[test_case(4)]
#[test_case(42)]
#[test_case(123)]
fn test_across_boundary(halvingnum: u64) {
    let zec_blocks_per_btc_blocks = 8;
    let hh1 = halving_height(halvingnum);
    let start = hh1 - 2 * zec_blocks_per_btc_blocks;
    let end = hh1 + 2 * zec_blocks_per_btc_blocks;

    let raw_points = (start..end).map(|h| (idealtime::at(h), NU5.block_subsidy(h)));
    let bucket_points = TimeBucketIter::new(raw_points, idealtime::bitcoin_block_target());

    let buckets: Vec<(idealtime::DateTime, Zat)> = bucket_points.collect();

    assert_eq!(buckets.len(), 4);
    assert_eq!(buckets[0].1, buckets[1].1);
    assert_eq!(buckets[2].1, buckets[3].1);
    assert_eq!(buckets[0].1 / 2, buckets[2].1);
}
