struct VaultState {
    pub shares_total: u64,
    pub token_total: u64,
}

pub fn mul_div_floor(a: u64, b: u64, c: u64) -> u64 {
    (a as u128)
        .checked_mul(b as u128)
        .unwrap()
        .checked_div(c as u128)
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn mul_div_ceil(a: u64, b: u64, c: u64) -> u64 {
    (a as u128)
        .checked_mul(b as u128)
        .unwrap()
        .div_ceil(c as u128)
        .try_into()
        .unwrap()
}

macro_rules! require_gt {
    ($lhs: expr, $rhs: expr) => {
        if $lhs > $rhs {
        } else {
            panic!()
        }
    };
}

impl VaultState {
    pub fn deposit(&mut self, tkn: u64) -> u64 {
        let shares_for_user = if self.shares_total == self.token_total {
            tkn
        } else {
            mul_div_floor(tkn, self.shares_total, self.token_total)
        };

        self.mint_shares(shares_for_user);
        self.add_token(tkn);

        shares_for_user
    }

    pub fn withdraw(&mut self, shares: u64) -> u64 {
        let tkn_for_user = if self.shares_total == self.token_total {
            shares
        } else {
            mul_div_floor(shares, self.token_total, self.shares_total)
        };

        self.burn_shares(shares);
        self.del_token(tkn_for_user);
        tkn_for_user
    }

    pub fn reward(&mut self, tkn: u64) {
        self.add_token(tkn)
    }

    pub fn slash(&mut self, tkn: u64) {
        self.del_token(tkn)
    }

    fn burn_shares(&mut self, amt: u64) {
        self.shares_total = self.shares_total.checked_sub(amt).unwrap();
    }

    fn mint_shares(&mut self, amt: u64) {
        require_gt!(amt, 0);
        self.shares_total = self.shares_total.checked_add(amt).unwrap();
    }

    fn add_token(&mut self, amt: u64) {
        require_gt!(amt, 0);
        self.token_total = self.token_total.checked_add(amt).unwrap();
    }

    fn del_token(&mut self, amt: u64) {
        self.token_total = self.token_total.checked_sub(amt).unwrap();
    }
}

mod fv {
    use super::*;
    use cvlr::mathint::NativeInt as MathInt;
    use cvlr::prelude::*;

    struct FvVaultState {
        shares_total: MathInt,
        token_total: MathInt,
    }

    impl FvVaultState {
        pub fn new(vault: &VaultState) -> Self {
            Self {
                shares_total: vault.shares_total.into(),
                token_total: vault.token_total.into(),
            }
        }

        pub fn assume_solvency(&self) {
            let v = self;
            cvlr_assume!(v.shares_total <= v.token_total);
        }

        pub fn check_solvency(&self) {
            let v = self;
            cvlr_assert!(v.shares_total <= v.token_total);
        }

        pub fn check_fixed_rate(&self, old: &FvVaultState) {
            let new = self;
            cvlr_assert_eq!(
                old.token_total * new.shares_total,
                new.token_total * old.shares_total
            );
        }

        pub fn check_no_dilution(&self, old: &FvVaultState) {
            let new = self;
            cvlr_assert_le!(
                old.token_total * new.shares_total,
                new.token_total * old.shares_total
            );
        }
    }

    impl From<&VaultState> for FvVaultState {
        fn from(vault: &VaultState) -> Self {
            Self::new(vault)
        }
    }

    impl cvlr::nondet::Nondet for VaultState {
        fn nondet() -> Self {
            Self {
                token_total: nondet(),
                shares_total: nondet(),
            }
        }
    }

    impl cvlr::log::CvlrLog for FvVaultState {
        #[inline(always)]
        fn log(&self, tag: &str, logger: &mut cvlr::log::CvlrLogger) {
            use cvlr::log::cvlr_log_with;
            cvlr_log_with("", &tag, logger);
            cvlr_log_with("\ttoken_total", &self.token_total, logger);
            cvlr_log_with("\tshares_total", &self.shares_total, logger);
        }
    }

    #[rule]
    pub fn rule_vault_solvency_withdraw() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();
        fv_vault_pre.assume_solvency();

        let shares_arg: u64 = nondet();
        let _ = vault.withdraw(shares_arg);
        clog!(stringify!(vault.withdraw(shares_arg);));

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, shares_arg, fv_vault_post);

        fv_vault_post.check_solvency();
    }

    #[rule]
    pub fn rule_vault_solvency_deposit() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();
        fv_vault_pre.assume_solvency();

        let token_arg: u64 = nondet();
        let _ = vault.deposit(token_arg);
        clog!(stringify!(vault.deposit(token_arg);));

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, token_arg, fv_vault_post);

        fv_vault_post.check_solvency();
    }

    #[rule]
    pub fn rule_vault_solvency_reward() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();
        fv_vault_pre.assume_solvency();

        let token_arg: u64 = nondet();
        let _ = vault.reward(token_arg);
        clog!(stringify!(vault.reward(token_arg);));

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, token_arg, fv_vault_post);

        fv_vault_post.check_solvency();
    }

    #[rule]
    pub fn rule_vault_solvency_slash() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();
        fv_vault_pre.assume_solvency();

        let token_arg: u64 = nondet();
        let _ = vault.slash(token_arg);

        clog!(stringify!(vault.slash(token_arg);));

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, token_arg, fv_vault_post);

        fv_vault_post.check_solvency();
    }

    #[rule]
    pub fn rule_vault_no_dilution_withdraw() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();

        let shares_arg: u64 = nondet();
        let out = vault.withdraw(shares_arg);
        clog!(stringify!(vault.withdraw(shares_arg)));
        clog!(out);

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, shares_arg, fv_vault_post);

        fv_vault_post.check_no_dilution(&fv_vault_pre);
    }

    #[rule]
    pub fn rule_vault_no_dilution_deposit() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();

        let token_arg: u64 = nondet();
        let out = vault.deposit(token_arg);
        clog!(stringify!(vault.deposit(token_arg)));
        clog!(out);

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, token_arg, fv_vault_post);

        fv_vault_post.check_no_dilution(&fv_vault_pre);
    }

    #[rule]
    pub fn rule_vault_no_dilution_reward() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();

        let token_arg: u64 = nondet();
        let out = vault.reward(token_arg);
        clog!(stringify!(vault.reward(token_arg)));
        clog!(out);

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, token_arg, fv_vault_post);

        fv_vault_post.check_no_dilution(&fv_vault_pre);
    }

    #[rule]
    pub fn rule_vault_no_dilution_slash() {
        let mut vault: VaultState = nondet();

        let fv_vault_pre: FvVaultState = (&vault).into();

        let token_arg: u64 = nondet();
        let out = vault.slash(token_arg);
        clog!(stringify!(vault.reward(token_arg)));
        clog!(out);

        let fv_vault_post: FvVaultState = (&vault).into();

        clog!(fv_vault_pre, token_arg, fv_vault_post);

        fv_vault_post.check_no_dilution(&fv_vault_pre);
    }
}
