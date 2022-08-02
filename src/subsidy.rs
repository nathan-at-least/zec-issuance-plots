mod btc;
mod nu5;
mod zpf;

use crate::units::{Height, Zat};

/// A subsidy scheme
#[derive(Debug)]
pub enum Subsidy {
    /// The subsidy scheme defined by Zcash as of NU5. (See Warning.)
    ///
    /// WARNING: This is not yet guaranteed to be consensus compatible. Unit tests exist for
    /// various properties, but an input->output compatible verification against zcashd doesn't yet
    /// exist.
    NU5,
    Btc,

    /// Posterity Fund proposal at the given activation height.
    PosterityFund(Height),
}
use Subsidy::*;

impl Subsidy {
    pub fn legend(&self) -> String {
        match self {
            NU5 => "ZEC (as of NU5)".to_string(),
            Btc => "BTC".to_string(),
            PosterityFund(h) => format!("ZEC (Posterity Fund proposal activating at height {})", h),
        }
    }
}

pub struct SubsidyGenerator {
    height: Height,
    subsidy: Subsidy,
    supply: Zat,
}

impl IntoIterator for Subsidy {
    type Item = (Height, Zat, Zat);
    type IntoIter = SubsidyGenerator;

    fn into_iter(self) -> SubsidyGenerator {
        SubsidyGenerator {
            height: 0,
            subsidy: self,
            supply: 0,
        }
    }
}

impl Iterator for SubsidyGenerator {
    type Item = (Height, Zat, Zat);

    fn next(&mut self) -> Option<Self::Item> {
        let h = self.height;
        let s = self.get_subsidy();
        self.height += 1;
        self.supply += s;
        Some((h, s, self.supply))
    }
}

impl SubsidyGenerator {
    fn get_subsidy(&self) -> Zat {
        match self.subsidy {
            NU5 => self::nu5::block_subsidy(self.height),
            Btc => self::btc::block_subsidy(self.height),
            PosterityFund(activation_height) => {
                if self.height < activation_height {
                    self::nu5::block_subsidy(self.height)
                } else {
                    self::zpf::block_subsidy(self.supply)
                }
            }
        }
    }
}
