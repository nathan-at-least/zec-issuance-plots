use crate::halving::halving_height;
use crate::idealtime;
use crate::subsidy::Subsidy::NU5;
use crate::timebuckets::TimeBucketIter;
use crate::units::Zat;

#[test]
fn test_across_boundary() {
    let zec_blocks_per_btc_blocks = 8;
    let hh1 = halving_height(1);
    let start = hh1 - zec_blocks_per_btc_blocks;
    let end = hh1 + zec_blocks_per_btc_blocks;

    let raw_points = (start..end).map(|h| (idealtime::at(h), NU5.block_subsidy(h)));
    let bucket_points = TimeBucketIter::new(raw_points, idealtime::bitcoin_block_target());

    let buckets: Vec<(idealtime::DateTime, Zat)> = bucket_points.collect();

    assert_eq!(buckets.len(), 2);
    assert_eq!(buckets[0].1 / 2, buckets[1].1);
}
