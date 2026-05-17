# Architecture

> System design and contract interaction flow for `ledja-contracts`.

## Overview

`ledja-contracts` is a Soroban smart contract suite composed of four independent contracts and one shared crate. Each contract compiles to its own `.wasm` binary and owns its own on-chain storage. Cross-contract communication is intentional and minimal.

```
ledja-contracts/
├── contracts/
│   ├── invoice/     — payment requests between seller and buyer
│   ├── payroll/     — recurring salary disbursements
│   ├── expense/     — business cost recording
│   └── inventory/   — stock movement tracking
└── shared/          — common types, enums, and errors
```

---

## Contract Interaction Map

```
[invoice] ──on settlement──► [inventory]   stock levels updated on invoice settlement
[invoice] ──on creation───► [expense]      optional: auto-log invoice as a business expense
[payroll] ──standalone──────►              no cross-contract deps at v1
[expense] ──standalone──────►              records only; optionally linked to an invoice ID
```

At v1, `payroll` and `expense` are fully standalone. Cross-contract calls from `invoice` to `inventory` and `expense` are the only inter-contract dependencies.

---

## Authorization Model

| Action                  | Who Can Call           |
|-------------------------|------------------------|
| `create_invoice`        | Any authenticated user |
| `settle_invoice`        | Buyer only             |
| `add_payroll_recipient` | Admin only             |
| `run_payroll`           | Admin only             |
| `record_expense`        | Any authenticated user |
| `record_stock_movement` | Admin only             |

Authorization is enforced via Soroban's `require_auth()` pattern on every restricted entrypoint.

---

## Storage Layout

- Each contract owns its own ledger storage — no shared entries between contracts.
- Unbounded lists are never stored in a single ledger entry; use keyed entries per record.
- Events are emitted for all state-changing operations (settlement, disbursement, stock movement).
- Shared types (structs, enums, errors) live in the `shared` crate and are imported by each contract.

---

## Fee Routing

- **Invoice settlement**: deducts a protocol fee and routes it to a treasury address stored in contract storage.
- **Payroll run**: charges a flat execution fee routed to the same treasury.
- Fee percentages are configurable by admin at deploy time and stored in contract storage.

---

## Soroban-Specific Constraints

- Each contract compiles independently to a `.wasm` binary via `cargo build --target wasm32-unknown-unknown --release`.
- Ledger entry size limits apply — avoid storing unbounded collections in a single entry.
- Use `soroban-sdk` events for anything that needs to be indexed off-chain by `ledja-indexer`.
- Cross-contract calls use Soroban's `invoke_contract` — keep them minimal to reduce gas costs.

---

## Naming Conventions

| Scope       | Convention  | Example              |
|-------------|-------------|----------------------|
| Files       | `snake_case` | `lib.rs`, `types.rs` |
| Types       | `PascalCase` | `Invoice`, `Payroll` |
| Functions   | `snake_case` | `create_invoice`     |
| Storage keys | `SCREAMING_SNAKE_CASE` | `INVOICE_KEY` |

---

## Glossary

- **Invoice**: A payment request from seller to buyer with a due date and amount.
- **Payroll run**: A batch disbursement of salaries to registered recipients.
- **Expense**: A recorded business cost linked to a category and optionally an invoice.
- **Stock movement**: A stock-in or stock-out event tied to an invoice settlement.
- **Treasury**: The protocol-controlled address that receives fee revenue.

---

> This is a living document. Update it as the architecture evolves. All cross-contract logic changes must reference this document.
