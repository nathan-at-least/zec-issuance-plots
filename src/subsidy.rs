mod nu5;

use crate::units::{Height, Zat};

/// A subsidy scheme
#[derive(Debug)]
pub enum Subsidy {
    /// The subsidy scheme defined by Zcash as of NU5 (WARNING: This is not yet guaranteed to be
    /// consensus compatible.)
    NU5,
}

impl Subsidy {
    /// Calculate the block subsidy for a given height for the scheme
    pub fn block_subsidy(&self, height: Height) -> Zat {
        use Subsidy::*;

        match self {
            NU5 => self::nu5::block_subsidy(height),
        }
    }
}
