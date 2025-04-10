/// A very simple example of fee computation
/// To be used in slides to show one application of CVLR
///
/// prover link: https://prover.certora.com/output/175561/0c2af99e624c4bc1b70139b1536a4a3d?anonymousKey=eaa2ee5205b275102bb3a5c1576b131568f3fc76
use cvlr::prelude::*;

#[rule]
pub fn rule_very_simple_fee() {
    check_fee(nondet(), nondet());
}

fn check_fee(amt: u64, bps: u16) {
    // require that amt is greater than 0
    cvlr_assume!(amt > 0);
    // require that bps is in the valid range
    cvlr_assume!(bps <= 10_000);
    // calculate the fee based on the amount and basis points
    let fee = (amt as u128)
        .checked_mul(bps as u128)
        .and_then(|x| x.div_ceil(10_000).into())
        .unwrap();
    // check that fee does not round to 0
    if bps > 0 {
        cvlr_assert_gt!(fee, 0);
    }
}

#[rule]
pub fn rule_very_simple_fee_bad() {
    check_fee_bad(nondet(), nondet());
}

fn check_fee_bad(amt: u64, bps: u16) {
    // require that amt is greater than 0
    cvlr_assume!(amt > 0);
    // require that bps is in the valid range
    cvlr_assume!(bps <= 10_000);
    // calculate the fee based on the amount and basis points
    let fee = (amt as u128)
        .checked_mul(bps as u128)
        .and_then(|x| x.checked_div(10_000))
        .unwrap();
    // check that fee does not round to 0
    if bps > 0 {
        cvlr_assert_gt!(fee, 0);
    }
}
