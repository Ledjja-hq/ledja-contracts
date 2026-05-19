#![no_std]
use shared::LedjaError;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ExpenseContract;

#[contractimpl]
impl ExpenseContract {}
