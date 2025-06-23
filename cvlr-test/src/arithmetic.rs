use cvlr::prelude::*;
use cvlr_fixed::native_fixed::NativeFixedU128;
use cvlr::mathint::NativeInt;


#[rule]
pub fn nested_div_commute() {
    let a: u64 = nondet();
    let b: u64 = nondet();
    let c: u64 = nondet();

    cvlr_assume!(b > 0);
    cvlr_assume!(c > 0);
    let r1 = a.div_ceil(b).div_ceil(c);
    let r2 = a.div_ceil(c).div_ceil(b);

    clog!(a, b, c);
    cvlr_assert_eq!(r1, r2);
}

#[rule]
pub fn nested_floor_div_commute() {
    let a: u64 = nondet();
    let b: u64 = nondet();
    let c: u64 = nondet();

    cvlr_assume!(b > 0);
    cvlr_assume!(c > 0);
    let r1 = a.checked_div(b).unwrap().checked_div(c).unwrap();
    let r2 = a.checked_div(c).unwrap().checked_div(b).unwrap();

    clog!(a, b, c);
    cvlr_assert_eq!(r1, r2);
}

/// Fixedpoint number
type FpNum = NativeFixedU128<60>;

fn mul_div_ceil_slow(f: FpNum, x: u64, y: u64) -> NativeInt {
    let f_bits = NativeInt::from(f.to_bits());
    let muldiv_bits = (f_bits * x).div_ceil(y.into());
    FpNum::from_bits(muldiv_bits.into()).to_ceil()
}

fn mul_div_ceil_fast(f: FpNum, x: u64, y: u64) -> NativeInt {
    // factor f into whole and fractional parts
    let a = f.to_floor();
    let b = f - FpNum::from(a);

    (a*x + (b*x).to_ceil()).div_ceil(y.into())
}

#[rule]
pub fn mul_div_ceil_of_frac() {
    let f:  FpNum = nondet();
    let x: u64 = nondet();
    let y: u64 = nondet();
    cvlr_assume!(y > 0);

    clog!(f, x, y);
    let r1 = mul_div_ceil_slow(f, x, y);
    let r2 = mul_div_ceil_fast(f, x, y);
    cvlr_assert_eq!(r1, r2);
}