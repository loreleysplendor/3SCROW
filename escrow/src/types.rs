use candid::{CandidType, Nat, Principal};

pub type DepositReceipt = Result<Nat, DepositErr>;

pub type WithdrawReceipt = Result<Nat, WithdrawErr>;

#[derive(CandidType)]
pub struct Balance {
    pub owner: Principal,
    pub token: Principal,
    pub amount: Nat,
}

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

#[derive(CandidType)]
pub enum WithdrawErr {
    BalanceLow,
    TransferFailure,
}

#[derive(CandidType)]
pub enum GeneralErr {
    CallFailure,
}
