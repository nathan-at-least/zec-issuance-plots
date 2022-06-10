use crate::consts::post_blossom_halving_interval;
use crate::units::{Halvings, Height};

pub fn halving_height(halvings: Halvings) -> Height {
    if halvings.into() == 0 {
        Height::from(0)
    } else {
        let first_halving = Height::from(1_046_400);

        first_halving + (halvings - 1) * post_blossom_halving_interval()
    }
}
