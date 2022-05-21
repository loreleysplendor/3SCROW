export LEDGER_ID=$(dfx canister id ledger)

dfx deploy escrow --argument "(opt principal \"$LEDGER_ID\")"
