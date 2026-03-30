//! # HarvestSend Contract
//!
//! A Soroban smart contract for cross-border remittance income logging.
//!
//! ## Purpose
//!
//! HarvestSend creates a tamper-proof on-chain income history for farmers.
//! Instead of storing personal information, it stores only wallet-linked
//! financial records that can be verified by partners and institutions.
//!
//! ## Flow
//!
//! 1. **Register Farmer** — a farmer wallet is registered once.
//! 2. **Log Income** — a remittance income entry is appended for that farmer.
//! 3. **Aggregate Totals** — total farmer income is updated on each entry.
//! 4. **Read Data** — total income and complete history are queryable.
//!
//! ## Stored Data
//!
//! - `IncomeRecord`: farmer address, amount, token symbol, timestamp.
//! - `FarmerTotalIncome`: running total of all recorded income.
//! - `FarmerIncomeHistory`: full list of income entries per farmer.
//!
//! ## Key Invariants
//!
//! - A farmer cannot be registered twice.
//! - Income can only be logged for a registered farmer.
//! - Income amount must be strictly greater than zero.
//! - `total_income` is the sum of all logged records for that farmer.
//! - No personal data (name, address text, location, etc.) is stored on-chain.
//!
//! ## Events
//!
//! - `income_logged`: emitted after a successful income log.
//!
#![no_std]

#[cfg(test)]
extern crate std;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone)]
pub struct IncomeRecord {
	pub farmer_address: Address,
	pub amount: i128,
	pub token_symbol: Symbol,
	pub timestamp: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
	FarmerRegistered(Address),
	FarmerTotalIncome(Address),
	FarmerIncomeHistory(Address),
}

#[contract]
pub struct HarvestSendContract;

#[contractimpl]

impl HarvestSendContract {
	// Registers a farmer address and prevents duplicate registrations.
	pub fn register_farmer(env: Env, farmer_address: Address) {
		let storage = env.storage().persistent();
		let farmer_key = DataKey::FarmerRegistered(farmer_address.clone());

		if storage.has(&farmer_key) {
			panic!("farmer already registered");
		}

		storage.set(&farmer_key, &true);
		storage.set(&DataKey::FarmerTotalIncome(farmer_address.clone()), &0_i128);
		storage.set(
			&DataKey::FarmerIncomeHistory(farmer_address),
			&Vec::<IncomeRecord>::new(&env),
		);
	}

	// Logs incoming remittance as farm income for a registered farmer.
	// Requires a positive amount and updates both history and total income.
	pub fn log_income(env: Env, farmer_address: Address, amount: i128, token_symbol: Symbol) {
		if amount <= 0 {
			panic!("amount must be greater than zero");
		}

		let storage = env.storage().persistent();
		let farmer_key = DataKey::FarmerRegistered(farmer_address.clone());

		if !storage.has(&farmer_key) {
			panic!("farmer is not registered");
		}

		let record = IncomeRecord {
			farmer_address: farmer_address.clone(),
			amount,
			token_symbol,
			timestamp: env.ledger().timestamp(),
		};

		let history_key = DataKey::FarmerIncomeHistory(farmer_address.clone());
		let mut history: Vec<IncomeRecord> = storage.get(&history_key).unwrap_or(Vec::new(&env));
		history.push_back(record.clone());
		storage.set(&history_key, &history);

		let total_key = DataKey::FarmerTotalIncome(farmer_address.clone());
		let current_total: i128 = storage.get(&total_key).unwrap_or(0_i128);
		let new_total = current_total + amount;
		storage.set(&total_key, &new_total);

		env.events().publish(
			(Symbol::new(&env, "income_logged"), farmer_address),
			(amount, new_total),
		);
	}

	// Returns the total recorded income for a farmer.
	pub fn get_total_income(env: Env, farmer_address: Address) -> i128 {
		env.storage()
			.persistent()
			.get(&DataKey::FarmerTotalIncome(farmer_address))
			.unwrap_or(0_i128)
	}

	// Returns the full on-chain income history for a farmer.
	pub fn get_income_history(env: Env, farmer_address: Address) -> Vec<IncomeRecord> {
		env.storage()
			.persistent()
			.get(&DataKey::FarmerIncomeHistory(farmer_address))
			.unwrap_or(Vec::new(&env))
	}
}

	#[cfg(test)]
	mod test;
