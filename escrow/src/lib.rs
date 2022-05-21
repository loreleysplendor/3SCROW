use std::cell::RefCell;

use candid::{candid_method, export_service, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountIdentifier, Memo, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};

mod types;
mod utils;

use types::*;
use utils::principal_to_subaccount;

const ICP_FEE: u64 = 10_000;

// https://github.com/dfinity/examples/blob/master/motoko/defi/architecture.md

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>,
}

#[init]
fn init(ledger: Option<Principal>) {
    ic_cdk::setup();
    STATE.with(|s| {
        s.borrow_mut().owner = Some(caller());
        s.borrow_mut().ledger = ledger;
    });
}

#[update]
pub async fn deposit(_token_canister_id: Option<Principal>) -> DepositReceipt {
    let caller = caller();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);
    // if _token_canister_id for support for DIP20 tokens
    let amount = deposit_icp(caller).await?;
    DepositReceipt::Ok(amount)
}

async fn deposit_icp(caller: Principal) -> Result<Nat, DepositErr> {
    let canister_id = ic_cdk::api::id();
    let _ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));

    let balance_args = ic_ledger_types::AccountBalanceArgs { account };
    let balance = ic_ledger_types::account_balance(_ledger_canister_id, balance_args)
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    if balance.e8s() < ICP_FEE {
        return Err(DepositErr::BalanceLow);
    }

    Ok((balance.e8s() - ICP_FEE).into())
}

#[update(name = "getDepositAddress")]
pub fn get_deposit_address() -> AccountIdentifier {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());

    AccountIdentifier::new(&canister_id, &subaccount)
}

#[update]
pub fn string_address() -> String {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());

    AccountIdentifier::new(&canister_id, &subaccount).to_string()
}
