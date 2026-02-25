## RSA + Proof-of-Work Practice in Rust

This project (`c2`) demonstrates how to combine **asymmetric cryptography (RSA)** with a simple **Proof-of-Work (PoW)** mechanism in Rust.

### What this program does

1. **Generate an RSA keypair**  
   - Creates a 2048-bit RSA **private key** and the corresponding **public key**.

2. **Run a PoW search (4 leading zeros)**  
   - Repeatedly computes `SHA-256(nickname + nonce)` using a given nickname (e.g. `"Eric"`).  
   - Increments `nonce` until the hash (in hexadecimal) starts with **4 leading zeros**.  
   - Once found, prints:
     - Time spent searching  
     - The `nonce`  
     - The input string `nickname + nonce`  
     - The hash value

3. **Sign the PoW result with the RSA private key**  
   - Takes the successful input string (`nickname + nonce` that satisfied the PoW condition).  
   - Uses the RSA **private key** to compute a digital signature over this string.

4. **Verify the signature with the RSA public key**  
   - Uses the RSA **public key** to verify that the signature is valid for the original `nickname + nonce`.  
   - Demonstrates:
     - A successful verification for the correct data.  
     - A failed verification when the data is tampered (e.g. using a different nonce).

This shows the typical workflow:

- **PoW**: proves that some computational work has been done.  
- **RSA signatures**: prove that the work result comes from the holder of a specific private key.

### Project structure

- `src/main.rs`  
  - Coordinates the overall flow:
    - Generates the RSA keypair.  
    - Runs PoW for a 4-zero SHA-256 hash.  
    - Signs the PoW input with the private key.  
    - Verifies the signature using the public key.  
    - Shows that tampering with the data causes verification to fail.

- `src/pow.rs`  
  - Implements the PoW search logic (`find_hash_with_prefix`).  
  - Returns a `PowResult` containing the successful `nickname + nonce`, nonce value, hash, and elapsed time.

- `src/rsa.rs`  
  - Generates the RSA keypair (`generate_rsa_keypair`).  
  - Signs data with the private key (`sign_data`).  
  - Verifies signatures with the public key (`verify_signature`).

### Prerequisites

- Rust toolchain (`cargo`, `rustc`) installed.  
  See `https://www.rust-lang.org/learn/get-started`.

### How to run

From the `c2` directory:

```bash
cargo run
```

You can edit the nickname in `src/main.rs` (`let nickname = "Eric";`) to your own nickname, then re-run to see different PoW inputs and signatures.

