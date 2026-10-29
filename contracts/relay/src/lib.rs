#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Address, Env};

#[contract]
pub struct Relay;

#[contractimpl]
impl Relay {
    // Takes the counter contract's ID, so it is deployed after the counter.
    pub fn __constructor(env: Env, counter: Address) {
        env.storage().instance().set(&symbol_short!("COUNTER"), &counter);
    }

    // Calls the counter's bump, so its cost includes the counter's.
    pub fn relay(env: Env) -> u32 {
        let counter: Address = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap();
        env.invoke_contract(&counter, &symbol_short!("bump"), vec![&env])
    }
}
