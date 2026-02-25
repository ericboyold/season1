## Proof-of-Work (PoW) Practice in Rust

This project (`c1`) is a simple Proof-of-Work (PoW) practice program written in Rust.  
It repeatedly computes SHA-256 hashes over a string composed of **your nickname + a nonce**, incrementing the nonce until the hash satisfies a given difficulty target.

### What it does

- **Task 1**:  
  Continually modifies the `nonce` and computes `SHA-256(nickname + nonce)` until it finds a hash that starts with **4 leading zeros** in hexadecimal.  
  When found, it prints:
  - Time spent searching  
  - The input string (`nickname + nonce`)  
  - The hash value

- **Task 2**:  
  After Task 1, it continues to search for a hash that starts with **5 leading zeros** in hexadecimal.  
  When found, it prints:
  - Time spent searching  
  - The input string (`nickname + nonce`)  
  - The hash value

### How it works (conceptually)

1. Choose a **nickname** (e.g. `"Eric"` in `main.rs`).
2. Start from `nonce = 0` and construct the data string:  
   `data = nickname + nonce`.
3. Compute the SHA-256 hash of `data`.
4. Convert the hash to a hexadecimal string.
5. Check whether the hash starts with the required number of zeros (4 or 5).  
   - If it does **not** match, increase the nonce and repeat.  
   - If it **does** match, record the elapsed time and print the results.

This mimics the core idea of PoW systems (like cryptocurrency mining), where finding a valid hash is **computationally expensive**, but verifying it is easy.

### Project structure

- `src/main.rs`  
  - Sets your nickname.  
  - Runs two PoW tasks: one for 4 leading zeros, one for 5 leading zeros.

- `src/pow.rs`  
  - Implements the `find_hash_with_prefix` function which performs the actual PoW search loop and logging.

### Prerequisites

- Rust toolchain (`cargo`, `rustc`) installed.  
  You can install Rust from `https://www.rust-lang.org/learn/get-started`.

### How to run

From the `c1` directory:

```bash
cargo run
```

You can edit the nickname in `src/main.rs` (`let nickname = "Eric";`) to your own nickname and re-run the program to see different results.

### Notes

- The time to find a valid hash grows as you increase the required number of leading zeros (difficulty).  
- Progress logs (e.g. every 1,000,000 attempts) are printed so you can see the search advancing.