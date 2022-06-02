mod nu5;

use crate::units::{Height, Zat};

#[derive(Debug)]
pub enum Subsidy {
    NU5,
}

impl Subsidy {
    pub fn block_subsidy(&self, height: Height) -> Zat {
        use Subsidy::*;

        match self {
            NU5 => self::nu5::block_subsidy(height),
        }
    }
}
