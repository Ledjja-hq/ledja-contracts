#![no_std]
use shared::LedjaError;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {}
