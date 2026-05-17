#![no_std]
use soroban_sdk::{contracttype, Address, Symbol};

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

#[contracttype]
#[derive(Clone)]
pub struct PayrollRecord {
    pub recipient: Address,
    pub amount: i128,
    pub frequency_days: u64,
    pub last_paid: u64,
}

#[contracttype]
#[derive(Clone)]
pub struct Expense {
    pub id: u64,
    pub submitter: Address,
    pub amount: i128,
    pub category: Symbol,
    pub timestamp: u64,
    pub linked_invoice_id: Option<u64>,
}

#[contracttype]
#[derive(Clone)]
pub struct InventoryItem {
    pub sku: Symbol,
    pub quantity: i128,
    pub linked_invoice_id: Option<u64>,
}
