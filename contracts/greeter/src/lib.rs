#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec};

#[contract]
pub struct Greeter;

#[contractimpl]
impl Greeter {
    // Takes text, so the fixture has to pass an argument that contains a space.
    pub fn hello(_env: Env, to: String) -> u32 {
        to.len()
    }

    // Takes a list, passed as JSON with spaces.
    pub fn total(_env: Env, values: Vec<u32>) -> u32 {
        values.iter().sum()
    }
}
