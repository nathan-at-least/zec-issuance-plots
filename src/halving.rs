use crate::consts::POST_BLOSSOM_HALVING_INTERVAL;
use crate::units::Height;

const FIRST_HALVING: Height = 1_046_400;

pub fn halving_height(halvingnum: usize) -> Height {
    if halvingnum == 0 {
        0
    } else {
        FIRST_HALVING + (halvingnum - 1) * POST_BLOSSOM_HALVING_INTERVAL
    }
}
