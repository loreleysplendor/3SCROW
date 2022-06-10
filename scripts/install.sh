dfx start --background --clean --host 127.0.0.1:8000

# use this file to install any weird stuff such as the ledger...

### === DEPLOY LOCAL LEDGER =====
dfx identity new minter
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)

dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)

# Use private api for install
rm ledger/ledger.did
cp ledger/ledger.private.did ledger/ledger.did

# dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'

dfx deploy ledger --argument '(record  {
    minting_account = "'${MINT_ACC}'";
    initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; };
    send_whitelist = vec {}
    })'
export LEDGER_ID=$(dfx canister id ledger)

# Replace with public api
rm ledger/ledger.did
cp ledger/ledger.public.did ledger/ledger.did

echo "=== DEPLOYED LEDGER CANISTER ==="

# ### === DEPLOY DIP TOKENS =====

# dfx canister create AkitaDIP20
# dfx canister create GoldenDIP20
# dfx build AkitaDIP20
# dfx build GoldenDIP20

# export ROOT_PRINCIPAL="principal \"$(dfx identity get-principal)\""
# dfx canister install GoldenDIP20 --argument="(\"https://dogbreedslist.com/wp-content/uploads/2019/08/Are-Golden-Retrievers-easy-to-train.png\", \"Golden Coin\", \"GLD\", 8, 10000000000000000, $ROOT_PRINCIPAL, 10000)"
# dfx canister install AkitaDIP20 --argument="(\"https://akitagoose.com/wp-content/uploads/2021/12/IMG_0674.png\", \"Akita Coin\", \"AKI\", 8, 10000000000000000, $ROOT_PRINCIPAL, 10000)"

# # set fees 
# dfx canister call AkitaDIP20 setFeeTo "($ROOT_PRINCIPAL)"
# dfx canister call AkitaDIP20 setFee "(420)" 
# dfx canister call GoldenDIP20 setFeeTo "($ROOT_PRINCIPAL)"
# dfx canister call GoldenDIP20 setFee "(420)" 

### === DEPLOY INTERNET IDENTITY =====

cd internet_identity
npm install
npm run build
cd ..

II_FETCH_ROOT_KEY=1 II_DUMMY_CAPTCHA=1 dfx deploy internet_identity --no-wallet --argument '(null)'
# II_ENV=development dfx deploy internet_identity --no-wallet --argument '(null)'
echo "=== DEPLOYED II CANISTER ==="

## === INSTALL FRONTEND / BACKEND ==== 

# escrow canister
dfx deploy escrow --argument "(opt principal \"$LEDGER_ID\")"

# webapp front end + back end
npm install
dfx deploy webapp_assets
echo "=== DEPLOYED WEBAPP CANISTERS ==="

# rsync -avr .dfx/$(echo ${DFX_NETWORK:-'**'})/canisters/** --exclude='assets/' --exclude='idl/' --exclude='*.wasm' --delete src/frontend/declarations

# dfx canister create frontend
# pushd src/frontend
# npm install
# npm run build
# popd
# dfx build frontend
# dfx canister install frontend

# echo "===== VISIT DEFI FRONTEND ====="
# echo "http://localhost:8000?canisterId=$(dfx canister id frontend)"
# echo "===== VISIT DEFI FRONTEND ====="
