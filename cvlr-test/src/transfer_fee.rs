use cvlr::mathint::NativeInt as MathInt;
use cvlr::prelude::*;
use std::cmp;

pub const MAX_FEE_BASIS_POINTS: u16 = 10_000;
const ONE_IN_BASIS_POINTS: u128 = MAX_FEE_BASIS_POINTS as u128;

/// Transfer fee information
pub struct TransferFee {
    /// First epoch where the transfer fee takes effect
    pub epoch: u64, // Epoch,
    /// Maximum fee assessed on transfers, expressed as an amount of tokens
    pub maximum_fee: u64,
    /// Amount of transfer collected as fees, expressed as basis points of the
    /// transfer amount, ie. increments of 0.01%
    pub transfer_fee_basis_points: u16,
}
impl TransferFee {
    /// Calculate ceiling-division
    ///
    /// Ceiling-division
    ///     `ceil[ numerator / denominator ]`
    /// can be represented as a floor-division
    ///     `floor[ (numerator + denominator - 1) / denominator]`
    fn ceil_div(numerator: u128, denominator: u128) -> Option<u128> {
        numerator
            .checked_add(denominator)?
            .checked_sub(1)?
            .checked_div(denominator)
    }

    /// Calculate the transfer fee
    pub fn calculate_fee(&self, pre_fee_amount: u64) -> Option<u64> {
        let transfer_fee_basis_points = self.transfer_fee_basis_points as u128;
        if transfer_fee_basis_points == 0 || pre_fee_amount == 0 {
            Some(0)
        } else {
            let numerator = (pre_fee_amount as u128).checked_mul(transfer_fee_basis_points)?;
            clog!(MathInt::from(numerator));
            let raw_fee = Self::ceil_div(numerator, ONE_IN_BASIS_POINTS)?
                .try_into() // guaranteed to be okay
                .ok()?;

            clog!(raw_fee);
            Some(cmp::min(raw_fee, self.maximum_fee))
        }
    }
}

impl cvlr::log::CvlrLog for TransferFee {
    fn log(&self, tag: &str, logger: &mut cvlr::log::CvlrLogger) {
        use cvlr::log::cvlr_log_with;
        clog!(self.epoch => "epoch" ; logger);
        cvlr_log_with("", &tag, logger);
        cvlr_log_with("\tepoch", &self.epoch, logger);
        cvlr_log_with("\tmaximum_fee", &self.maximum_fee, logger);
        cvlr_log_with(
            "\ttransfer_fee_basis_points",
            &self.transfer_fee_basis_points,
            logger,
        );
    }
}

#[rule]
pub fn rule_monotonicity_of_calculate_fee() {
    let pre_fee_amount_x: u64 = nondet();
    let pre_fee_amount_y: u64 = nondet();
    let tf_bps: u16 = nondet();
    let maximum_fee: u64 = nondet();

    let tf = TransferFee {
        epoch: nondet(),
        maximum_fee,
        transfer_fee_basis_points: tf_bps,
    };
    clog!(tf);

    // to simplify TAC debugging
    cvlr_assume!(tf_bps > 0);
    cvlr_assume!(pre_fee_amount_x > 0);
    cvlr_assume!(pre_fee_amount_y > 0);

    cvlr_assume!(tf_bps <= MAX_FEE_BASIS_POINTS);
    cvlr_assume!(pre_fee_amount_x > pre_fee_amount_y);

    clog!(pre_fee_amount_x, pre_fee_amount_y);

    let fee_x = tf.calculate_fee(pre_fee_amount_x).unwrap();
    let fee_y = tf.calculate_fee(pre_fee_amount_y).unwrap();
    cvlr_assert_ge!(fee_x, fee_y);
}
