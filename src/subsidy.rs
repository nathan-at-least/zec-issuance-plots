mod nu5;

use crate::units::{Height, Zat};

/// A subsidy scheme
#[derive(Debug)]
pub enum Subsidy {
    /// The subsidy scheme defined by Zcash as of NU5 (WARNING: This is not yet guaranteed to be
    /// consensus compatible.)
    NU5,
}
use Subsidy::*;

impl Subsidy {
    /// Calculate the block subsidy for a given height for the scheme
    pub fn block_subsidy(&self, height: Height) -> Zat {
        match self {
            NU5 => self::nu5::block_subsidy(height),
        }
    }

    /// The maximum supply for the issuance schedule, if any
    pub fn max_supply(&self) -> Option<Zat> {
        match self {
            NU5 => Some(self::nu5::max_supply()),
        }
    }
}
