# 3SCROW

An escrow service platform designed on the internet computer.

This project allows users to escrow funds from multiple crypto
chains and tokens upon the IC network.

---

## Quickstart

### Useful Links So Far:

- [internet identity](https://github.com/dfinity/internet-identity)
- [motoko defi example](https://github.com/dfinity/examples/tree/master/motoko/defi)
- [rust defi example](https://github.com/dfinity/examples/tree/master/rust/defi)
- [internet computer docs](https://smartcontracts.org/docs/current/developer-docs/ic-overview)

### Requirements

- [dfx](https://smartcontracts.org/docs/developers-guide/install-upgrade-remove.html)
- [cmake](https://cmake.org/)

Rust and the wasm compile target.

```
rustup target add wasm32-unknown-unknown
```

### Installation

```bash
# clone the repo
git clone --recurse-submodules --shallow-submodules git@github.com:SmellyFilly/3SCROW.git
make install
```

```bash
# only submodules
git submodule update --init --recursive
```

### Ledger Usage

Give yourself icp tokens:

```bash
make init-local II_PRINCIPAL=<YOUR II PRINCIPAL>
```

### Internet Identity Usage

Run Internet Identity front end:

```bash
cd interent_identity/
npm start
```

Create an anchor by going to the front end...
Then, for testing, get your principal by:
