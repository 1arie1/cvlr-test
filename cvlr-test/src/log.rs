use cvlr::prelude::*;

#[rule]
pub fn log_i128() {
    let x: i128 = nondet();

    cvlr_assume!(x <= ((u64::MAX >> 1) as i128));
    clog!((x >> 64) as u64, x as u64);
    clog!(I128(&x));
    cvlr_assert!(x > 5);
}

struct I128<'a>(pub &'a i128);

impl<'a> cvlr::log::CvlrLog for I128<'a> {
    fn log(&self, tag: &str, logger: &mut cvlr::log::CvlrLogger) {
        let lo = *self.0 as u64;
        let hi = (*self.0 >> 64) as u64;
        logger.log_u64_2(tag, hi, lo);
    }
 
}