use super::{
    BLOSSOM_ACTIVATION, POST_BLOSSOM_HALVING_INTERVAL, PRE_BLOSSOM_HALVING_INTERVAL, START_SUBSIDY,
    SUBSIDY_SLOW_START_INTERVAL, SUBSIDY_SLOW_START_SHIFT,
};
use crate::units::{Height, Zat};
use test_case::test_case;

const FIRST_HALVING: Height = 1_046_400;
const SECOND_HALVING: Height = FIRST_HALVING + POST_BLOSSOM_HALVING_INTERVAL;
const PENULTIMATE_HALVING: Height = FIRST_HALVING + 28 * POST_BLOSSOM_HALVING_INTERVAL;
const ULTIMATE_HALVING: Height = FIRST_HALVING + 29 * POST_BLOSSOM_HALVING_INTERVAL;
const SIXTY_FIFTH_HALVING: Height = FIRST_HALVING + 64 * POST_BLOSSOM_HALVING_INTERVAL;

/// This amount is the increment in subsidy per block during slow start:
const SLOW_START_INCREMENT: Zat = START_SUBSIDY / SUBSIDY_SLOW_START_INTERVAL;

// Define boundary values around constants:
const FIRST_HALVING_MINUS_1: Height = FIRST_HALVING - 1;
const FIRST_HALVING_PLUS_1: Height = FIRST_HALVING + 1;

const SECOND_HALVING_MINUS_1: Height = SECOND_HALVING - 1;
const SECOND_HALVING_PLUS_1: Height = SECOND_HALVING + 1;

const SUBSIDY_SLOW_START_SHIFT_MINUS_1: Height = SUBSIDY_SLOW_START_SHIFT - 1;
const SUBSIDY_SLOW_START_SHIFT_PLUS_1: Height = SUBSIDY_SLOW_START_SHIFT + 1;

const SUBSIDY_SLOW_START_INTERVAL_MINUS_1: Height = SUBSIDY_SLOW_START_INTERVAL - 1;
const SUBSIDY_SLOW_START_INTERVAL_PLUS_1: Height = SUBSIDY_SLOW_START_INTERVAL + 1;

const BLOSSOM_ACTIVATION_MINUS_1: Height = BLOSSOM_ACTIVATION - 1;
const BLOSSOM_ACTIVATION_PLUS_1: Height = BLOSSOM_ACTIVATION + 1;

const PRE_BLOSSOM_HALVING_INTERVAL_MINUS_1: Height = PRE_BLOSSOM_HALVING_INTERVAL - 1;
const PRE_BLOSSOM_HALVING_INTERVAL_PLUS_1: Height = PRE_BLOSSOM_HALVING_INTERVAL + 1;

const POST_BLOSSOM_HALVING_INTERVAL_MINUS_1: Height = POST_BLOSSOM_HALVING_INTERVAL - 1;
const POST_BLOSSOM_HALVING_INTERVAL_PLUS_1: Height = POST_BLOSSOM_HALVING_INTERVAL + 1;

#[test_case(
    0 => (0, 0)
)]
#[test_case(
    1 => (0, SLOW_START_INCREMENT)
)]
#[test_case( // Notice the gap between SUBSIDY_SLOW_START_SHIFT and one block prior:
    SUBSIDY_SLOW_START_SHIFT => (0, START_SUBSIDY / 2 + SLOW_START_INCREMENT)
)]
#[test_case(
    SUBSIDY_SLOW_START_SHIFT_MINUS_1 => (0, START_SUBSIDY / 2 - SLOW_START_INCREMENT)
)]
#[test_case(
    SUBSIDY_SLOW_START_SHIFT_PLUS_1 => (0, START_SUBSIDY / 2 + 2 * SLOW_START_INCREMENT)
)]
#[test_case(
    SUBSIDY_SLOW_START_INTERVAL => (0, START_SUBSIDY)
)]
#[test_case(
    SUBSIDY_SLOW_START_INTERVAL_MINUS_1 => (0, START_SUBSIDY)
)]
#[test_case(
    SUBSIDY_SLOW_START_INTERVAL_PLUS_1 => (0, START_SUBSIDY)
)]
#[test_case(
    BLOSSOM_ACTIVATION => (0, START_SUBSIDY / 2)
)]
#[test_case(
    BLOSSOM_ACTIVATION_MINUS_1 => (0, START_SUBSIDY)
)]
#[test_case(
    BLOSSOM_ACTIVATION_PLUS_1 => (0, START_SUBSIDY / 2)
)]
#[test_case(
    PRE_BLOSSOM_HALVING_INTERVAL => (0, START_SUBSIDY / 2)
)]
#[test_case(
    PRE_BLOSSOM_HALVING_INTERVAL_MINUS_1 => (0, START_SUBSIDY / 2)
)]
#[test_case(
    PRE_BLOSSOM_HALVING_INTERVAL_PLUS_1 => (0, START_SUBSIDY / 2)
)]
#[test_case(
    POST_BLOSSOM_HALVING_INTERVAL => (1, START_SUBSIDY / 4)
)]
#[test_case(
    POST_BLOSSOM_HALVING_INTERVAL_MINUS_1 => (1, START_SUBSIDY / 4)
)]
#[test_case(
    POST_BLOSSOM_HALVING_INTERVAL_PLUS_1 => (1, START_SUBSIDY / 4)
)]
#[test_case(
    FIRST_HALVING => (1, START_SUBSIDY / 4)
)]
#[test_case(
    FIRST_HALVING_MINUS_1 => (0, START_SUBSIDY / 2)
)]
#[test_case(
    FIRST_HALVING_PLUS_1 => (1, START_SUBSIDY / 4)
)]
#[test_case(
    SECOND_HALVING => (2, START_SUBSIDY / 8)
)]
#[test_case(
    SECOND_HALVING_MINUS_1 => (1, START_SUBSIDY / 4)
)]
#[test_case(
    SECOND_HALVING_PLUS_1 => (2, START_SUBSIDY / 8)
)]
#[test_case(
    PENULTIMATE_HALVING => (29, 1)
)]
#[test_case(
    ULTIMATE_HALVING => (30, 0)
)]
#[test_case(
    SIXTY_FIFTH_HALVING => (65, 0)
)]
fn halvings_and_subsidy(height: Height) -> (u64, Zat) {
    (super::halvings_at(height), super::block_subsidy(height))
}
