// shared/src/lib.rs
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Sent,
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
#[derive(Clone, Debug, PartialEq)]
pub enum LedjaError {
    InvalidAmount,
    InvalidDueDate,
    SellerBuyerConflict,
    InvoiceNotFound,
}