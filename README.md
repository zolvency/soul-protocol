# Soul Protocol

## Overview

Soul Protocol is the core identity management layer of Zolvency. It implements the concept of "SoulIDs", acting as a secure and verifiable on-chain identity for users within the Soroban (Stellar) ecosystem.

This repository contains the Soroban smart contracts required to mint, manage, and recover Soul identities. The contract handles the association between Stellar accounts, device passkeys (secp256r1), and their on-chain representation.

## Structure

- `contracts/soul/`: The main Soroban smart contract for the Soul Identity system.
  - `src/lib.rs`: The main contract interface and entry points.
  - `src/logic.rs`: Core logic for minting, recovering, and managing souls.
  - `src/storage.rs`: Helper functions for persistent and instance storage management.
  - `src/test.rs`: Unit tests for the contract.

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2021)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

## Building

To build the contract, run:

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing

To run the unit tests, use:

```bash
cargo test
```

## Features

1. **Soul Minting**: Create a new Soul identity associated with a Stellar owner address, authenticated with a biometric passkey.
2. **Passkey Recovery**: Update the primary passkey using a pre-defined recovery key and cryptographic signature (secp256r1).
3. **Identity Verification**: Look up Soul IDs by address or passkey, ensuring tight coupling between wallets and biometric identities.
4. **Relayer Support**: Authorized transactions via specific relayer addresses to streamline onboarding and reduce friction for end users.

## License

MIT
