/// Example of a specification
use cvlr::prelude::*;

/// Function under verification
fn compute_fee(amount: u64, fee_bps: u16) -> Result<u64,()> {
    if amount > 0 {
        Ok((amount as u128)
            .checked_mul(fee_bps as u128)
            .and_then(|x| Some(x.div_ceil(10_000)))
            .ok_or(())?
            .try_into()
            .map_err(|_| ())?)
    } else {
        Err(())
    }
}

#[rule]
pub fn rule_fee_sanity_ok() {
    compute_fee(nondet(), nondet()).unwrap();
    cvlr_satisfy!(true);
}

#[rule]
pub fn rule_fee_assessed_ok() {
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
pub fn rule_fee_liveness_ok() {
    let amt: u64 = nondet();
    let fee_bps: u16 = nondet();
    cvlr_assume!(fee_bps <= 10_000);
    let fee = compute_fee(amt, fee_bps);
    clog!(amt, fee_bps, fee);
    if fee.is_err() {
        cvlr_assert!(amt == 0);
    }
}
