## Season 1 – Rust Cryptography & Blockchain Practice

This repository contains a series of small Rust projects (`c1`, `c2`, `c3`) designed to practice and illustrate core concepts in **Proof-of-Work (PoW)**, **public-key cryptography (RSA)**, and **blockchain data structures**.

Each subproject is a self-contained Cargo crate with its own `README.md` and focuses on a specific concept. Together, they form a progressive learning path:

- From basic PoW hash search
- To combining PoW with RSA signatures
- To building a minimal Proof-of-Work blockchain

---

### Project Overview

- **`c1` – Proof-of-Work (PoW) Practice**  
  A minimal PoW implementation:
  - Repeatedly computes `SHA-256(nickname + nonce)` using your nickname.
  - Searches for hashes with a required number of leading zeros (difficulty).
  - Demonstrates:
    - How increasing difficulty makes finding a valid hash more time-consuming.
    - That verification of a found hash is trivial compared to the search.
  - See `c1/README.md` for detailed behavior and usage.

- **`c2` – RSA + PoW Integration**  
  Combines asymmetric cryptography with PoW:
  - Generates a 2048-bit RSA public/private keypair.
  - Runs a PoW search (4 leading zeros) similar to `c1`.
  - Signs the successful PoW input (`nickname + nonce`) with the RSA private key.
  - Verifies the signature with the corresponding public key.
  - Demonstrates:
    - Successful verification for correct data.
    - Failed verification when the data is tampered (e.g., changing the nonce).
  - See `c2/README.md` for detailed behavior and usage.

- **`c3` – Simple Blockchain Demo**  
  Implements a minimal blockchain with PoW:
  - Defines `Transaction`, `Block`, and `Blockchain` structures.
  - Uses SHA-256 over JSON-serialized blocks to compute hashes.
  - Mines new blocks using a difficulty based on leading zeros (PoW).
  - Validates the entire chain and demonstrates how tampering with historical data breaks chain validity.
  - See `c3/README.md` for detailed behavior and usage.

---

### Tech Stack

All subprojects use:

- **Language**: Rust (Edition 2024)
- **Hashing**: `sha2` crate (SHA-256)

Additional crates per project:

- `c1`:
  - `sha2`
- `c2`:
  - `sha2` – PoW hashing
  - `rsa` – RSA key generation, signing, verification
  - `rand` – randomness for RSA
- `c3`:
  - `chrono` – timestamps for blocks
  - `sha2` – block hashing
  - `serde`, `serde_json` – struct serialization to JSON

---

### How to Run the Projects

From the `season1` directory, each project lives in its own subfolder (`c1`, `c2`, `c3`).

Run them individually with Cargo:

```bash
# c1 – Proof-of-Work practice
cd c1
cargo run

# c2 – RSA + PoW
cd ../c2
cargo run

# c3 – Simple blockchain demo
cd ../c3
cargo run
```

For more details (program flow, outputs, and configuration notes), refer to the `README.md` inside each subproject directory.

---

### Learning Goals

By going through `c1` → `c2` → `c3`, you can:

- Understand how PoW hash puzzles work and why difficulty matters.
- See how **digital signatures** (RSA) can be used to prove ownership of a PoW result.
- Learn the basic structure and behavior of a **PoW-based blockchain**, including:
  - Block linkage via hashes
  - Mining via Proof-of-Work
  - Chain validation and tamper detection.

