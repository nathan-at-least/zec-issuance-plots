// use crate::units::Height;
use derive_more::{From, Into};
use num_derive::NumOps;

/// An amount of zatoshi
#[derive(Debug, NumOps, From, Into)]
pub struct Zat(usize);

pub const COIN: Zat = Zat(100_000_000);
