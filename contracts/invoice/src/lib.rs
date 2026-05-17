#![no_std]
use soroban_sdk::{contract, contractimpl, Env};
use shared::InvoiceStatus;

#[contract]
pub struct InvoiceContract;

#[contractimpl]
impl InvoiceContract {
    pub fn default_status(_env: Env) -> InvoiceStatus {
        InvoiceStatus::Draft
    }
}
