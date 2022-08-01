use crate::units::{Height, Sat};

pub fn block_subsidy(h: Height) -> Sat {
    let initial = 50 * 100_000_000;
    let halvings = h / 210_000;

    let mut subsidy = initial;
    for _ in 0..halvings {
        subsidy /= 2;
    }

    subsidy
}
