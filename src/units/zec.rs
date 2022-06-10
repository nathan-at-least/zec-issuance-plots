/// An amount of ZEC in Zatoshi precision.
#[derive(Copy, Clone, Debug)]
pub struct Zec(u64);

/// Transcription from `zcash/src/amount.h` `COIN`
const ZATOSHI_PER_ZEC: u64 = 100_000_000;

/// No amount should ever be equal or greater than this number of Zatoshi, because the issuance
/// curve sums to an amount slightly less than this due to integer division rounding.
const SUPPLY_CAP: u64 = 21_000_000 * ZATOSHI_PER_ZEC;

impl Zec {
    pub const fn from_zatoshi(zat: u64) -> Self {
        assert!(
            zat < SUPPLY_CAP,
            "{:?} >= {:?} (SUPPLY CAP)",
            zat,
            SUPPLY_CAP
        );
        Zec(zat)
    }
}
