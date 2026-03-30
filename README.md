# HarvestSend

A Soroban smart contract for verifiable farm-income records on Stellar.

HarvestSend logs remittance income events as immutable on-chain records linked to farmer wallet addresses. The goal is to provide transparent, auditable income history that can be used by cooperatives, lenders, and ecosystem partners.

---

## Current Status

- Smart contract: implemented
- Testnet deployment: live
- Unit tests: passing
- Frontend: not built yet

Note: this repository has a frontend folder scaffold, but there is currently no connected user interface for the contract.

---

## Features

- Farmer registration with duplicate protection
- Income logging with strict positive amount validation
- On-chain running total per farmer
- Full per-farmer income history retrieval
- Event emission on successful income logs (`income_logged`)
- No personal profile fields stored on-chain

---

## How It Works

```text
Register farmer address
				|
				v
Log remittance income (amount, token symbol)
				|
				+--> append income record to farmer history
				|
				+--> update farmer total income
				|
				v
Read total income or full history anytime
```

---

## Contract Interface

### `register_farmer`

```rust
pub fn register_farmer(env: Env, farmer_address: Address)
```

Registers a farmer once and initializes:

- `FarmerTotalIncome` to `0`
- `FarmerIncomeHistory` to empty list

### `log_income`

```rust
pub fn log_income(env: Env, farmer_address: Address, amount: i128, token_symbol: Symbol)
```

Logs one income event for a registered farmer.

Validation rules:

- `amount > 0`
- farmer must already be registered

Side effects:

- append new `IncomeRecord`
- increment total income
- emit `income_logged` event

### `get_total_income`

```rust
pub fn get_total_income(env: Env, farmer_address: Address) -> i128
```

Returns the farmer's aggregated income total.

### `get_income_history`

```rust
pub fn get_income_history(env: Env, farmer_address: Address) -> Vec<IncomeRecord>
```

Returns all income records for the farmer.

---

## Data Model

### `IncomeRecord`

```rust
pub struct IncomeRecord {
		pub farmer_address: Address,
		pub amount: i128,
		pub token_symbol: Symbol,
		pub timestamp: u64,
}
```

### Persistent Keys

- `FarmerRegistered(Address)`
- `FarmerTotalIncome(Address)`
- `FarmerIncomeHistory(Address)`

---

## Key Invariants

- A farmer cannot be registered twice
- Income cannot be logged for unregistered farmers
- Income amount must always be strictly positive
- Stored total equals sum of logged entries per farmer
- Personal details are not stored on-chain

---

## Testnet Deployment

- Contract ID: `CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5`
- Contract page:
	- https://lab.stellar.org/r/testnet/contract/CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5
- WASM upload tx:
	- https://stellar.expert/explorer/testnet/tx/bd8b594df4fa7347e29834182ab67fca973c93a927692a8a173f0b49c432be2e
- Deploy tx:
	- https://stellar.expert/explorer/testnet/tx/bced5a92e373b83b21f5e8f69439c9444d562f0f6d50d8bf8f0468b5a74c8f29

---

## Getting Started

### Prerequisites

- Rust toolchain
- Stellar CLI
- Soroban-compatible build target

```bash
rustup target add wasm32v1-none
```

### Build

```bash
cargo build --target wasm32v1-none --release
```

Expected artifact:

```text
target/wasm32v1-none/release/harvestsend_contract.wasm
```

### Test

```bash
cargo test
```

### Deploy (example)

```bash
stellar keys generate alvin --overwrite --fund -n testnet

stellar contract deploy \
	--wasm target/wasm32v1-none/release/harvestsend_contract.wasm \
	--source alvin \
	--network testnet \
	--alias harvestsend-contract
```

---

## Quick CLI Usage

Register a farmer:

```bash
stellar contract invoke \
	--id CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5 \
	--source alvin \
	--network testnet \
	-- register_farmer \
	--farmer_address <FARMER_ADDRESS>
```

Log income:

```bash
stellar contract invoke \
	--id CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5 \
	--source alvin \
	--network testnet \
	-- log_income \
	--farmer_address <FARMER_ADDRESS> \
	--amount 1500 \
	--token_symbol USDC
```

Get total income:

```bash
stellar contract invoke \
	--id CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5 \
	--source alvin \
	--network testnet \
	-- get_total_income \
	--farmer_address <FARMER_ADDRESS>
```

---

## Project Structure

- `src/lib.rs` - contract logic
- `src/test.rs` - unit tests
- `frontend/` - frontend scaffold (not connected yet)

---

## Roadmap

- Build and integrate frontend dashboard
- Add indexer-backed analytics views
- Add stronger authorization model for approved income reporters
- Add end-to-end script tests for contract invoke flows

---

## License

MIT
