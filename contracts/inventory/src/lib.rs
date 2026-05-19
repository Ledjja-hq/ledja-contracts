#![no_std]
use shared::LedjaError;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {}
