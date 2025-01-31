use solana_program::account_info::AccountInfo;
use cvlr::asserts::cvlr_assert;

#[cvlr::rule]
pub fn ag_accounts() {
    let acc0: AccountInfo = cvlr_solana::cvlr_new_account_info();
    let acc1: AccountInfo = cvlr_solana::cvlr_new_account_info();
    let acc0_: AccountInfo = cvlr_solana::cvlr_new_account_info();
    let acc0c = acc0.clone();

    let lamports0 = acc0.lamports();
    let lamports1 = acc1.lamports();
    let lamports0_ = acc0_.lamports();
    let lamports0c = acc0c.lamports();



    cvlr_assert!(lamports0c == lamports0);
    cvlr_assert!(lamports0_ == lamports0);
    cvlr_assert!(lamports0 == lamports1);
}