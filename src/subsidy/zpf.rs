use crate::consts::COIN;
use crate::units::{Height, Zat};

const SUPPLY_CAP: Zat = 21_000_000 * COIN;

pub fn block_subsidy(supply: Zat) -> Zat {
    let fund_balance = SUPPLY_CAP - supply;
    ((fund_balance as f64) * remainder_proportion()) as Zat
}

fn remainder_proportion() -> f64 {
    use once_cell::sync::OnceCell;

    static CELL: OnceCell<f64> = OnceCell::new();

    *CELL.get_or_init(|| {
        // (1-x)^blocks = 0.5
        // 1-x = 0.5^(1/half_life)
        // x = 1-0.5^(1/half_life)
        //
        const HALF_LIFE: Height = 840_000 * 4;
        dbg!(1.0f64 - dbg!(0.5f64.powf(1.0f64 / (HALF_LIFE as f64))))
    })
}
