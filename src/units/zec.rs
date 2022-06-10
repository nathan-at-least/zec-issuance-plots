use std::str::FromStr;

/// An amount of ZEC in Zatoshi precision.
#[derive(Copy, Clone, Debug)]
pub struct Zec(u64);

/// Transcription from `zcash/src/amount.h` `COIN`
const ZATOSHI_PER_ZEC: u64 = 100_000_000;

/// No amount should ever be equal or greater than this number of Zatoshi, because the issuance
/// curve sums to an amount slightly less than this due to integer division rounding.
const SUPPLY_CAP: u64 = 21_000_000 * ZATOSHI_PER_ZEC;

impl Zec {
    pub const fn one() -> Self {
        Zec::from_zatoshi(ZATOSHI_PER_ZEC)
    }

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

impl FromStr for Zec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (whole, frac) = s.split_once('.').unwrap_or((s, ""));
        let wholenum = u64::from_str(whole)?;

        if frac.len() > 8 {
            return Err("too much precision");
        }

        let mut fracpad = frac.to_string();
        while fracpad.len() < 8 {
            fracpad.push('0');
        }

        let fracnum = u64::from_str(&fracpad)?;

        Ok(Zec::from_zatoshi(wholenum * ZATOSHI_PER_ZEC + fracnum))
    }
}

// TODO: stop using `f32` altogether; right now it's an expedient for plotting.
impl From<Zec> for f32 {
    fn from(z: Zec) -> f32 {
        z.0 as f32 / ZATOSHI_PER_ZEC as f32
    }
}
