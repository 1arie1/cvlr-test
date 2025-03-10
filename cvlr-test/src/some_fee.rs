use cvlr::{nondet::nondet_with, prelude::*};

// #[cvlr::mock_fn(
    // with=crate::certora::mocks::some_fee::compute_fee)]
#[inline(never)]
pub fn compute_fee(amount: u64, fee_bps: u16) -> Option<u64> {
    let fee = amount.checked_mul(fee_bps as u64)?.checked_div(10_000)?;
    Some(fee)
}

#[rule]
pub fn check_compute_fee() {
    let amount: u64 = nondet();
    let fee_bps: u16 = nondet_with(|x| *x <= 10_000);
    let fee = compute_fee(amount, fee_bps).unwrap();
    clog!(amount, fee_bps, fee);
    cvlr_assert_le!(fee, amount);
    cvlr_assert_gt!(fee, 0);
}

#[rule]
pub fn check_mock_compute_fee() {
    let amount: u64 = nondet();
    let fee_bps: u16 = nondet_with(|x| *x <= 10_000);
    let fee = crate::certora::mocks::some_fee::compute_fee(amount, fee_bps).unwrap();
    clog!(amount, fee_bps, fee);
    cvlr_assert_le!(fee, amount);
    cvlr_assert_gt!(fee, 0);
}

extern "C" {
    fn CVT_register_mock_fn(fn_orig: usize, fn_mock: usize);
}

#[no_mangle]
pub fn setup_mock() {
    let fn1 = crate::some_fee::compute_fee as usize;
    let fn_mock = crate::certora::mocks::some_fee::compute_fee as usize; 

    unsafe { CVT_register_mock_fn(fn1, fn_mock);}
}
