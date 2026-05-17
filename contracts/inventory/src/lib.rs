#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {}
