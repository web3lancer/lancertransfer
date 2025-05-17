# 💸 Web3Lancer Payment & Escrow Module

This project implements a robust, extensible on-chain payment and escrow system, forming the foundation for the Web3Lancer platform's job, milestone, and reputation workflows. It is designed for real-world use, not as a demo or template.

## Features

- **Modular Payment Logic**: Core token transfer, milestone-based escrow, and extensible hooks for jobs, reviews, and disputes.
- **Delegation & Ephemeral Rollup Support**: Secure delegation and fast ephemeral execution for real-time payments.
- **Anchor & MagicBlock Integration**: Built with Anchor and MagicBlock's ephemeral rollup SDK for seamless Solana and rollup compatibility.

## Running Tests on Devnet


To run tests on the devnet, use the following command:

```bash
anchor test --skip-local-validator --skip-build --skip-deploy
```

## Running tests with a Local Ephemeral Rollup and Devnet

To run tests using a local ephemeral validator, follow these steps:

### 1. Install the Local Validator

```bash
npm install -g @magicblock-labs/ephemeral-validator
```

### 2. Start the Local Validator

```bash
ACCOUNTS_REMOTE=https://rpc.magicblock.app/devnet ACCOUNTS_LIFECYCLE=ephemeral ephemeral-validator
```

### 3. Run the Tests with the Local Validator

```bash
PROVIDER_ENDPOINT=http://localhost:8899 WS_ENDPOINT=ws://localhost:8900 anchor test --skip-build --skip-deploy --skip-local-validator
```

---

## Extending This Module

- Add new instructions in `src/milestone.rs` for milestone-based escrow and release.
- Add review and reputation logic in `src/review.rs`.
- Integrate with job, dispute, and governance modules as your platform grows.

---

This project is a core part of the [Web3Lancer](https://github.com/web3lancer/web3lancer) platform.
