# 3SCROW

An escrow service platform designed on the internet computer.

This project allows users to escrow funds from multiple crypto
chains and tokens upon the IC network.

---

## Quickstart

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
make install
```

### Ledger Usage

Give yourself icp tokens:

```bash
make init-local II_PRINCIPAL=<YOUR II PRINCIPAL>
```
