#![no_std]
use shared::{InvoiceStatus, LedjaError};
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct InvoiceContract;

#[contractimpl]
impl InvoiceContract {
    pub fn default_status(_env: Env) -> InvoiceStatus {
        InvoiceStatus::Draft
    }
}
