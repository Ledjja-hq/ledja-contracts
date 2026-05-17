# ledja-contracts

> Soroban smart contracts powering on-chain back-office automation for Nigerian SMEs on the Stellar blockchain.
>
> [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
> [![Built on Stellar](https://img.shields.io/badge/Built%20on-Stellar-blueviolet)](https://stellar.org)
> [![Soroban](https://img.shields.io/badge/Smart%20Contracts-Soroban-blue)](https://soroban.stellar.org)
>
> ## Overview
>
> ledja-contracts contains the Soroban smart contracts that form the trustless core of the Ledja platform. Written in Rust and deployed on the Stellar blockchain, these contracts handle all financial logic in a fully on-chain, auditable manner.
>
> By leveraging Soroban, Ledja eliminates the need for centralized servers for financial operations, enabling SMEs in Nigeria and across Africa to run compliant, transparent back-office processes without intermediaries.
>
> ## Contracts
>
> - invoice - Create, track, and settle invoices on-chain with configurable payment terms
> - - payroll - Automate recurring salary disbursements to multiple recipients
>   - - expense - Record and categorize business expenses with immutable audit trails
>     - - inventory - Track stock movements linked to on-chain invoice settlements
>      
>       - ## Tech Stack
>      
>       - - Language: Rust
>         - - Platform: Soroban (Stellar Smart Contracts)
>           - - SDK: soroban-sdk
>             - - Testing: Soroban test framework
>               - - Network: Stellar Testnet / Mainnet
>                
>                 - ## Getting Started
>                
>                 - Prerequisites: Rust (latest stable) and Soroban CLI
>                
>                 - ```bash
> cargo install --locked soroban-cli
> git clone https://github.com/Ledjja-hq/ledja-contracts.git
> cd ledja-contracts
> cargo build --target wasm32v1-none --release
> cargo test
> ```
>
> ## Ecosystem Fit and Monetization
>
> These contracts run natively on Stellar's Soroban platform, making Ledja one of the first open-source back-office suites built entirely on Stellar for emerging market SMEs.
>
> Monetization flows through the contracts via protocol fees on invoice settlements, payroll execution fees to sustain development, and enterprise features gated on-chain.
>
> ## Security
>
> All contracts are open-source and auditable. Input validation on all public entrypoints. Authorization checks using Soroban's require_auth() pattern. A third-party security audit is planned before mainnet launch.
>
> ## Contributing
>
> We welcome contributors especially those familiar with Rust and Soroban. Check our open issues for tasks labeled good first issue.
>
> ## Related Repositories
>
> - ledja-frontend: Web UI built with Next.js
> - ledja-indexer: Stellar blockchain indexer
> - ledja-sdk: TypeScript SDK for developers
>
> ## License
>
> MIT (c) Ledjja-hq
