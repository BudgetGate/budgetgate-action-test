#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
enum DataKey {
    Admin,
    Limit,
    Balance(Address),
}

#[contract]
pub struct Vault;

#[contractimpl]
impl Vault {
    // Runs once at deployment; deposit fails if it did not.
    pub fn __constructor(env: Env, admin: Address, limit: i128) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Limit, &limit);
    }

    // Needs the depositor's authorization; reads the limit and updates one balance.
    pub fn deposit(env: Env, from: Address, amount: i128) -> i128 {
        from.require_auth();
        let limit: i128 = env.storage().instance().get(&DataKey::Limit).unwrap();
        let key = DataKey::Balance(from);
        let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0) + amount;
        if balance > limit {
            panic!("over limit");
        }
        env.storage().persistent().set(&key, &balance);
        balance
    }
}
