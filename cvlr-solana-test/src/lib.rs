use cvlr::{rule, nondet, clog, asserts::*};

mod accounts;

#[rule]
pub fn test_satisfy() {
    let x: u64 = nondet();
    let y: u64 = nondet();

    cvlr_assume!(x > y);
    clog!(x, y);
    cvlr_satisfy!(x > y + 5);
}

#[rule]
pub fn test_assert_fail() {
    let x: u64 = nondet();
    let y: u64 = nondet();

    cvlr_assume!(x > y);
    clog!(x, y);
    cvlr_assert!(x > y + 5);
}