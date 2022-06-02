mod subsidy;
mod units;

use self::subsidy::Subsidy;

fn main() {
    dbg!(Subsidy::NU5.block_subsidy(0));
}
