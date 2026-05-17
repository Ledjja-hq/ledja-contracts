#![no_std]
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub enum InvoiceStatus {
    Draft,
    Issued,
    Paid,
    Overdue,
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
