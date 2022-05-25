use ic_cdk::caller;
use ic_cdk_macros::*;

#[query]
pub fn whoami() -> String {
    caller().to_text()
}
