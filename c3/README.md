## Simple Blockchain Demo in Rust

This project is a minimal blockchain implementation written in Rust.  
It demonstrates the core concepts behind a Proof-of-Work (PoW) blockchain:

- **Transactions**: transfers of value between a sender and a recipient
- **Blocks**: containers that bundle transactions together
- **Proof of Work**: a computational puzzle that miners must solve to create a valid block
- **Chain validation**: verifying that all blocks are linked correctly and satisfy the difficulty requirement
- **Tampering detection**: showing how modifying historical data invalidates the chain

### Features

- Defines a `Transaction` structure with `sender`, `recipient`, and `amount`.
- Defines a `Block` structure containing:
  - `index`
  - `timestamp`
  - a list of `transactions`
  - `proof` (nonce used for PoW)
  - `previous_hash` (hash of the previous block)
- Uses SHA-256 (`sha2` crate) to compute block hashes based on their serialized JSON (`serde` + `serde_json`).
- Maintains a `Blockchain` structure that:
  - starts with a **genesis block**
  - collects **pending transactions**
  - mines new blocks using a configurable **difficulty** (number of leading zeros in the hash)
  - validates the entire chain to ensure integrity
- In `main`, the program:
  - initializes a blockchain with difficulty 4
  - adds a few sample transactions
  - mines two blocks
  - validates the chain
  - then **simulates tampering** by changing the amount in an existing block and re-validates the chain to show it becomes invalid

### Requirements

- **Rust** toolchain (recommended: latest stable, e.g. 1.7x)

This crate uses the following dependencies (see `Cargo.toml`):

- `chrono` for timestamps
- `sha2` for SHA-256 hashing
- `serde` and `serde_json` for serialization

### How to Run

From the `c3` directory:

```bash
cargo run
```

You will see console output showing:

- mining progress (attempted proofs)
- successfully mined blocks and their contents
- validation results for the blockchain
- the effect of tampering with a block (the validation should fail after modification)

### Notes

- This implementation is for **educational and demonstration purposes only** and omits many features of real-world blockchains (networking, wallets, signatures, mempool, etc.).
- The difficulty is set to `4` leading zeros in `main.rs`. You can increase or decrease it to see how it affects mining time.

