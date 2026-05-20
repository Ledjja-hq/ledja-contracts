#![no_std]
use shared::{Expense, LedjaError};
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

#[contract]
pub struct ExpenseContract;

const EXPENSE_COUNT: Symbol = symbol_short!("EXP_CNT");

#[contractimpl]
impl ExpenseContract {
    pub fn record_expense(
        env: Env,
        submitter: Address,
        amount: i128,
        category: Symbol,
        linked_invoice_id: Option<u64>,
    ) -> Result<u64, LedjaError> {
        if amount <= 0 {
            return Err(LedjaError::InvalidAmount);
        }
        if category == Symbol::new(&env, "") {
            return Err(LedjaError::InvalidAmount);
        }

        let id: u64 = env.storage().instance().get(&EXPENSE_COUNT).unwrap_or(0) + 1;
        let expense = Expense {
            id,
            submitter,
            amount,
            category,
            timestamp: env.ledger().timestamp(),
            linked_invoice_id,
        };

        env.storage().persistent().set(&id, &expense);
        env.storage().instance().set(&EXPENSE_COUNT, &id);

        Ok(id)
    }
}
