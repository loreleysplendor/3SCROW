use candid::{candid_method, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;

const ANONYMOUS_SUFFIX: u8 = 4;

fn is_authorized_user() -> Result<(), String> {
    let principal = &caller();
    let bytes = principal.as_ref();

    match bytes.len() {
        1 if bytes[0] == ANONYMOUS_SUFFIX => Err("Anonymous principal not allowed".to_string()),
        _ => Ok(()),
    }
}

#[query]
fn get_principal() -> Principal {
    caller()
}

#[query]
fn get_canister_principal() -> Principal {
    ic_cdk::api::id()
}
