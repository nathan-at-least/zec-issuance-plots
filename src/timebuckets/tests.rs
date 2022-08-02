use crate::halving::halving_height;
use crate::idealtime::{bitcoin_block_target, Chain, DateTime, TimeModel};
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
    let zctime = TimeModel::new(Chain::Zcash);
    let zec_blocks_per_btc_blocks = 8;
    let hh1 = halving_height(halvingnum);
    let start = hh1 - 2 * zec_blocks_per_btc_blocks;
    let end = hh1 + 2 * zec_blocks_per_btc_blocks;

    let conv = |u| usize::try_from(u).unwrap();
    let raw_points = NU5
        .into_iter()
        .skip(conv(start))
        .take(conv(end - start))
        .map(|(h, zat, _)| (zctime.at(h), zat));
    let bucket_points = TimeBucketIter::new(raw_points, bitcoin_block_target());

    let buckets: Vec<(DateTime, Zat)> = bucket_points.collect();

    assert_eq!(buckets.len(), 4);
    assert_eq!(buckets[0].1, buckets[1].1);
    assert_eq!(buckets[2].1, buckets[3].1);
    assert_eq!(buckets[0].1 / 2, buckets[2].1);
}
