use solana_program::account_info::{AccountInfo, next_account_info};
use cvlr::prelude::*;
use cvlr_solana::{clog_acc_info, cvlr_deserialize_nondet_accounts};

#[rule]
pub fn clone_accounts() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();

    let acc0: &AccountInfo = account_info_iter.next().unwrap();
    let acc1: &AccountInfo = account_info_iter.next().unwrap();
    let acc0_: AccountInfo = acc0.clone();

    let lamports0 = acc0.lamports();
    let lamports1 = acc1.lamports();
    let lamports0_ = acc0_.lamports();

    cvlr_assert_eq!(lamports0_, lamports0);
    cvlr_assert_eq!(lamports0, lamports1);
}

#[rule]
pub fn init_accounts() {
    let account_infos = cvlr_deserialize_nondet_accounts();
    let account_info_iter = &mut account_infos.iter();

    let acc0: &AccountInfo = next_account_info(account_info_iter).unwrap();
    
    clog_acc_info!(acc0);

    cvlr_assume!(acc0.data_len() == 0);
    cvlr_assert!(acc0.data_is_empty());
    acc0.realloc(1024, false).unwrap();
    cvlr_assert_gt!(acc0.data_len(), 0);
    cvlr_assert_eq!(acc0.data_len(), 1024);

    acc0.realloc(2*1024, true).unwrap();
    cvlr_assert_eq!(acc0.data_len(), 2*1024);

    clog_acc_info!(acc0);
}