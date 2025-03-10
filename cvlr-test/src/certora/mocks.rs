pub mod some_fee {
    use cvlr::nondet::nondet_with;
    /// Mock of compute_fee
    ///
    /// Returns a value that is less than amount, to correspond to some fee that is
    /// <= 100%
    #[inline(never)]
    pub fn compute_fee(amount: u64, _fee_bps: u16) -> Option<u64> {
        let fee = nondet_with(|x: &u64| 0 < *x && *x <= amount);
        Some(fee)
    }
}
