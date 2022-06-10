use derive_more::{From, Into};

#[derive(Copy, Clone, Debug, From, Into)]
pub struct Seconds(u64);
