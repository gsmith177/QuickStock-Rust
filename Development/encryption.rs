
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;

const ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };
static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA512;

fn hash_password(password: &str, salt: &[u8]) -> Vec<u8> {
    let mut hash = [0u8; 64];
    pbkdf2::derive(DIGEST_ALG, ITERATIONS, salt, password.as_bytes(), &mut hash);
    hash.to_vec()
}

fn main() {
    let password = "my_password";
    let salt = b"some_salt";
    let hashed = hash_password(password, salt);
    println!("Hashed password: {:?}", hashed);
}
