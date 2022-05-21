use candid::{CandidType, Nat, Principal};

pub type DepositReceipt = Result<Nat, DepositErr>;

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}
