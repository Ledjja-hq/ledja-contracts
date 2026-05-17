#![no_std]
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum InvoiceStatus {
    Pending,
    Paid,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub struct Invoice {
    pub id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub amount: i128,
    pub due_date: u64,
    pub status: InvoiceStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct PayrollRecord {
    pub recipient: Address,
    pub amount: i128,
    pub frequency_days: u64,
    pub last_paid: u64,
}
