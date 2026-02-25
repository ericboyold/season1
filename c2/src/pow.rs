use std::time::Instant;
use sha2::{Sha256, Digest};

// PoW result returned from the PoW calculation
pub struct PowResult {
   pub input: String,
   pub nonce: u64,
   // hash: String,
   // elapsed_time: f64,
}

// Perform POW calculation
pub fn find_hash_with_prefix(nickname: &str, leading_zeros: usize) -> Option<PowResult> {
    let start_time = Instant::now();
    let target = "0".repeat(leading_zeros);

    println!("Start looking for hashes that begin with {} zeros...", leading_zeros);

    for nonce in 0..u64::MAX {
        // Construct the string data to be hashed
        let input = format!("{}{}", nickname, nonce);
        // Compute the SHA-256 hash
        let hash = Sha256::digest(input.as_bytes());
        // Convert the hash to a hexadecimal string
        let hash_hex = format!("{:x}", hash);
        // Check if the hash starts with a specified number of zeros.
        if hash_hex.starts_with(&target) {
            let elapsed_time = start_time.elapsed();

            println!("\nHash that meets the criteria was found");
            println!("Time taken: {:.2} seconds", elapsed_time.as_secs_f64());
            println!("Nonce: {}", nonce);
            println!("Hash data: {}", input);
            // Display the results in a more readable format
            let display_hash = format!("\x1b[32m{}\x1b[0m{}", &hash_hex[0..leading_zeros], &hash_hex[leading_zeros..hash_hex.len()]);

            println!("Hash: {}", display_hash);
            println!("Number of attempts: {}", nonce + 1);

            return Some(PowResult {
                input,
                nonce,
                // hash: hash_hex,
                // elapsed_time: elapsed_time.as_secs_f64(),
            });
        }

        // print progress every 1000000 attempts
        if nonce > 0 && nonce % 1_000_000 == 0 {
            println!("Attempts: {} million", nonce / 1_000_000);
        }
    }
    println!("Hash not found after {} attempts", u64::MAX);
    None
}