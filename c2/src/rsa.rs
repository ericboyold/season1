use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::signature::{Signer, Verifier, SignatureEncoding};
use rand::rngs::OsRng;
use sha2::Sha256;

// Generate RSA keypair
pub fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Private key generation failed.");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

// Sign data using the private key.
pub fn sign_data(private_key: &RsaPrivateKey, data: &str) -> Vec<u8> {
    // Create a signing key
    let signing_key = SigningKey::<Sha256>::new_unprefixed(private_key.clone());

    // Sign the data
    let signature = signing_key.sign(data.as_bytes());

    // Convert the signature to a byte vector
    signature.to_vec()
}

// Verify signature using public key.
pub fn verify_signature(public_key: &RsaPublicKey, data: &str, signature: &[u8]) -> bool {
    // Create a verification key
    let verifying_key = VerifyingKey::<Sha256>::new_unprefixed(public_key.clone());
    
    // Create a signature object
    let signature_obj = rsa::pkcs1v15::Signature::try_from(signature).expect("Invalid signature format");

    // Verify signature
    verifying_key.verify(data.as_bytes(), &signature_obj).is_ok()
}