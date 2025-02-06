/// Example of a specification
use cvlr::prelude::*;

/// Function under verification
fn compute_fee(amount: u64, fee_bps: u16) -> Result<u64,()> {
    if amount > 0 {
        Ok(amount
            .checked_mul(fee_bps as u64)
            .and_then(|x| x.checked_div(10_000))
            .ok_or(())?)
    } else {
        Err(())
    }
}

#[rule]
pub fn rule_fee_sanity() {
    compute_fee(nondet(), nondet()).unwrap();
    cvlr_satisfy!(true);
}

#[rule]
pub fn rule_fee_assessed() {
    let amt: u64 = nondet();
    let fee_bps: u16 = nondet();
    cvlr_assume!(fee_bps <= 10_000);
    let fee = compute_fee(amt, fee_bps).unwrap();
    clog!(amt, fee_bps, fee);
    cvlr_assert_le!(fee, amt);
    if fee_bps > 0 {
        cvlr_assert_gt!(fee, 0);
    }
}

#[rule]
pub fn rule_fee_liveness() {
    let amt: u64 = nondet();
    let fee_bps: u16 = nondet();
    cvlr_assume!(fee_bps <= 10_000);
    let fee = compute_fee(amt, fee_bps);
    clog!(amt, fee_bps, fee);
    if fee.is_err() {
        cvlr_assert!(amt == 0);
    }
}
