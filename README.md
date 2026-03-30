# HarvestSend Contract

HarvestSend is a Soroban smart contract on Stellar that records remittance income events for farmers as an auditable on-chain history.

The contract is designed to support trust and verification for cross-border payments by keeping financial records tamper-resistant and queryable.

## Current Status

- Smart contract: implemented and deployed to Stellar testnet
- Tests: implemented and passing (`cargo test`)
- Frontend: not built yet (no user-facing app connected to the contract at this time)

## Why This Contract

Many remittance workflows still rely on fragmented records. HarvestSend stores farmer-linked income logs on-chain so partner institutions can verify payment history without relying on mutable off-chain spreadsheets.

## What The Contract Stores

- IncomeRecord
	- farmer_address
	- amount
	- token_symbol
	- timestamp
- FarmerTotalIncome
	- running total per farmer
- FarmerIncomeHistory
	- full list of income records per farmer

## Contract Methods

- register_farmer(farmer_address)
	- Registers a farmer once
	- Initializes total income and history
- log_income(farmer_address, amount, token_symbol)
	- Requires farmer to be registered
	- Requires amount > 0
	- Appends history and updates total
	- Emits `income_logged` event
- get_total_income(farmer_address)
	- Returns aggregated total income
- get_income_history(farmer_address)
	- Returns full income log list

## Key Invariants

- Farmer cannot be registered twice
- Income cannot be logged for an unregistered farmer
- Income amount must be strictly positive
- Total income must match the sum of logged records
- No personal profile data is stored on-chain

## Tech Stack

- Rust (no_std contract)
- Soroban SDK `22.x`
- Stellar CLI

## Testnet Deployment

Deployed contract (testnet):

- Contract ID: `CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5`
- Contract page:
	- https://lab.stellar.org/r/testnet/contract/CAIICZKO6EVJL3GHTWGRKY6YUAKRSKV7JSH5FEUDZJQZ4WJVIGTG35K5

Deployment transactions:

- WASM upload tx:
	- https://stellar.expert/explorer/testnet/tx/bd8b594df4fa7347e29834182ab67fca973c93a927692a8a173f0b49c432be2e
- Contract deploy tx:
	- https://stellar.expert/explorer/testnet/tx/bced5a92e373b83b21f5e8f69439c9444d562f0f6d50d8bf8f0468b5a74c8f29

## Build And Deploy

1. Install target

```bash
rustup target add wasm32v1-none
```

2. Build contract

```bash
cargo build --target wasm32v1-none --release
```

3. Generate key (example alias: alvin)

```bash
stellar keys generate alvin --overwrite --fund -n testnet
```

4. Deploy

```bash
stellar contract deploy \
	--wasm target/wasm32v1-none/release/harvestsend_contract.wasm \
	--source alvin \
	--network testnet \
	--alias harvestsend-contract
```

## Local Testing

Run unit tests:

```bash
cargo test
```

Current coverage includes:

- Farmer registration initialization
- Income logging updates for totals and history
- Panic checks for duplicate registration
- Panic checks for unregistered farmer income logs
- Panic checks for non-positive income amounts

## Project Structure

- `src/lib.rs` - contract implementation
- `src/test.rs` - contract unit tests
- `frontend/` - placeholder web app scaffold (not integrated yet)

## Roadmap

- Build and connect a user-facing frontend
- Add role/authorization model for approved payment operators
- Add richer event indexing for analytics and reporting
- Add integration tests for CLI-based invoke flows
