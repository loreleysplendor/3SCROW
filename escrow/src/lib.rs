use std::cell::RefCell;
use std::convert::TryInto;

use candid::{candid_method, export_service, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use ic_ledger_types::{
    AccountBalanceArgs, AccountIdentifier, Memo, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

mod types;
mod utils;

use types::*;
use utils::principal_to_subaccount;

const ICP_FEE: u64 = 10_000;

// https://github.com/dfinity/examples/blob/master/motoko/defi/architecture.md

// Need to think about the signature flow. Look at some other applications
// just use principals for now. They seem to be unique per user.

// so have a func which is authorize(). this saves to the state that caller() x is authorizing
// the escrow.
// need to have a check to push back transactions if the cycles are close to running out on the canister

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Default)]
pub struct State {
    owner: Option<Principal>,
    ledger: Option<Principal>, // IC ledger
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
pub async fn deposit(_token_canister_id: Principal) -> DepositReceipt {
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

#[update(name = "withdraw")]
pub async fn withdraw(
    amount: Nat,
    _token_canister_id: Principal,
    address: Principal,
) -> WithdrawReceipt {
    let caller = caller();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    if _token_canister_id == ledger_canister_id {
        let account_id = AccountIdentifier::new(&address, &DEFAULT_SUBACCOUNT);
        withdraw_icp(&amount, account_id).await
    } else {
        ic_cdk::print("token");
        return Err(WithdrawErr::TransferFailure);
        // withdraw_token(token_canister_id, &amount, address).await
    }
}

async fn withdraw_icp(amount: &Nat, account_id: AccountIdentifier) -> Result<Nat, WithdrawErr> {
    let caller = caller();
    let canister_id = ic_cdk::api::id();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let escrow_account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));
    let transfer_amount = Tokens::from_e8s(
        (amount.to_owned() + ICP_FEE)
            .0
            .try_into()
            .map_err(|_| WithdrawErr::TransferFailure)?,
    );
    let balance = ic_ledger_types::account_balance(
        ledger_canister_id,
        AccountBalanceArgs {
            account: escrow_account,
        },
    )
    .await
    .map_err(|_| WithdrawErr::TransferFailure)?;
    if balance < transfer_amount {
        return Err(WithdrawErr::BalanceLow);
    }

    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount: transfer_amount,
        fee: Tokens::from_e8s(ICP_FEE),
        from_subaccount: Some(DEFAULT_SUBACCOUNT),
        to: account_id,
        created_at_time: None,
    };
    let icp_reciept = ic_ledger_types::transfer(ledger_canister_id, transfer_args)
        .await
        .map_err(|_| WithdrawErr::TransferFailure)
        .and_then(|v| v.map_err(|_| WithdrawErr::TransferFailure));

    ic_cdk::println!("Withdrawal of {} ICP to account {:?}", amount, &account_id);

    Ok(amount.to_owned() + ICP_FEE)
}

async fn get_escrow_balance(account: Principal) -> Result<Balance, GeneralErr> {
    let canister_id = ic_cdk::api::id();
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let escrow_account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&account));
    let balance = ic_ledger_types::account_balance(
        ledger_canister_id,
        AccountBalanceArgs {
            account: escrow_account,
        },
    )
    .await
    .map_err(|_| GeneralErr::CallFailure)?;

    Ok(Balance {
        owner: account,
        token: ledger_canister_id,
        amount: balance.e8s().into(),
    })
}

#[update]
pub fn string_address() -> String {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());

    AccountIdentifier::new(&canister_id, &subaccount).to_string()
}

#[query]
pub fn whoami() -> String {
    caller().to_text()
}

#[query]
pub fn ledger() -> String {
    let ledger_canister_id = STATE
        .with(|s| s.borrow().ledger)
        .unwrap_or(MAINNET_LEDGER_CANISTER_ID);
    ledger_canister_id.to_text()
}
