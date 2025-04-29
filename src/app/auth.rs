use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use rand::RngCore;
use serde::{Serialize, Deserialize};
use std::fs;
use base64::Engine;


const KEY: &[u8; 32] = b"verysecurekeymustbe32byteslong!!"; // now 32 bytes
const NONCE_SIZE: usize = 12;
const FILE_PATH: &str = "user_data.json";

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn save_credentials(username: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new_from_slice(KEY)?;
    let mut nonce = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce);

    let creds = Credentials {
        username: username.to_string(),
        password: password.to_string(),
    };

    let serialized = serde_json::to_string(&creds)?;
    let encrypted = match cipher.encrypt(Nonce::from_slice(&nonce), serialized.as_bytes()) {
        Ok(data) => data,
        Err(_) => return Err("Encryption failed".into()),
    };

    let payload = base64::engine::general_purpose::STANDARD.encode([&nonce[..], &encrypted[..]].concat());
    fs::write(FILE_PATH, payload)?;
    Ok(())
}
