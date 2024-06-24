use aes_gcm::{aead::{Aead, AeadCore, KeyInit}, Aes256Gcm, Key};
use pbkdf2::pbkdf2_hmac;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use std::collections::HashMap;

pub struct PasswordManager {
    master_key: Key<Aes256Gcm>,
    passwords: HashMap<String, (Vec<u8>, Vec<u8>)>,
}

impl PasswordManager {
    pub fn new(master_password: &str) -> Self {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            master_password.as_bytes(),
            &salt,
            10000, // number of iterations
            &mut key,
        );
        
        let master_key = Key::<Aes256Gcm>::from_slice(&key).clone();
        
        PasswordManager {
            master_key,
            passwords: HashMap::new(),
        }
    }

    pub fn add_password(&mut self, service: &str, password: &str) {
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        let cipher = Aes256Gcm::new(&self.master_key);
        let ciphertext = cipher.encrypt(&nonce, password.as_bytes())
            .expect("encryption failure!");

        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);

        self.passwords.insert(service.to_string(), (encrypted_data, salt.to_vec()));
    }

    pub fn get_password(&self, service: &str) -> Option<String> {
        if let Some((encrypted_data, _)) = self.passwords.get(service) {
            let (nonce, ciphertext) = encrypted_data.split_at(12);
            let cipher = Aes256Gcm::new(&self.master_key);
            let plaintext = cipher.decrypt(nonce.into(), ciphertext)
                .expect("decryption failure!");
            Some(String::from_utf8(plaintext).unwrap())
        } else {
            None
        }
    }
}