use crate::{HarvestSendContract, HarvestSendContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

#[test]
fn register_farmer_initializes_empty_state() {
	let env = Env::default();
	let contract_id = env.register(HarvestSendContract, ());
	let client = HarvestSendContractClient::new(&env, &contract_id);
	let farmer = Address::generate(&env);

	client.register_farmer(&farmer);

	assert_eq!(client.get_total_income(&farmer), 0);
	assert_eq!(client.get_income_history(&farmer).len(), 0_u32);
}

#[test]
fn log_income_updates_total_and_history() {
	let env = Env::default();
	let contract_id = env.register(HarvestSendContract, ());
	let client = HarvestSendContractClient::new(&env, &contract_id);
	let farmer = Address::generate(&env);
	let symbol = Symbol::new(&env, "USDC");

	client.register_farmer(&farmer);
	client.log_income(&farmer, &125_i128, &symbol);

	assert_eq!(client.get_total_income(&farmer), 125_i128);

	let history = client.get_income_history(&farmer);
	assert_eq!(history.len(), 1_u32);

	let first = history.get(0).expect("missing first income record");
	assert_eq!(first.farmer_address, farmer);
	assert_eq!(first.amount, 125_i128);
	assert_eq!(first.token_symbol, Symbol::new(&env, "USDC"));
}

#[test]
#[should_panic]
fn register_farmer_rejects_duplicates() {
	let env = Env::default();
	let contract_id = env.register(HarvestSendContract, ());
	let client = HarvestSendContractClient::new(&env, &contract_id);
	let farmer = Address::generate(&env);

	client.register_farmer(&farmer);
	client.register_farmer(&farmer);
}

#[test]
#[should_panic]
fn log_income_requires_registered_farmer() {
	let env = Env::default();
	let contract_id = env.register(HarvestSendContract, ());
	let client = HarvestSendContractClient::new(&env, &contract_id);
	let farmer = Address::generate(&env);

	client.log_income(&farmer, &10_i128, &Symbol::new(&env, "XLM"));
}

#[test]
#[should_panic]
fn log_income_requires_positive_amount() {
	let env = Env::default();
	let contract_id = env.register(HarvestSendContract, ());
	let client = HarvestSendContractClient::new(&env, &contract_id);
	let farmer = Address::generate(&env);

	client.register_farmer(&farmer);
	client.log_income(&farmer, &0_i128, &Symbol::new(&env, "XLM"));
}
