use aes_gcm::{aead::{Aead, AeadCore, KeyInit}, Aes256Gcm, Key};
use pbkdf2::pbkdf2_hmac;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use std::collections::HashMap;

use crate::util;

// TODO: add persistent master key
// (store encrypted data in file)
pub struct PasswordManager {
    username: String,
    master_key: Key<Aes256Gcm>,
    passwords: HashMap<String, (Vec<u8>, Vec<u8>)>,
}

impl PasswordManager {
    pub fn new(username: String, master_password: String) -> Self {

        // generate rand salt
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        
        // derive key from password input using PBKDF2
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(
            master_password.as_bytes(),
            &salt,
            10000,
            &mut key,
        );
        
        // create the key
        let master_key = Key::<Aes256Gcm>::from_slice(&key).clone();
        
        return Self {
            username,
            master_key,
            passwords: HashMap::new(),
        }
    }

    pub fn add_password(&mut self, service: String) {

        // prompt user for the password
        let password = util::prompt_service_password(service.clone());

        // generate rand salt for the
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        
        // generate one time rand val
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // create cipher instance w/ master key
        let cipher = Aes256Gcm::new(&self.master_key);
        
        // encrypt entry
        let ciphertext = cipher.encrypt(&nonce, password.as_bytes())
            .expect("encryption failure!");

        // concatenate nonce and ciphertext (same plaintext will map to diff ciphertext bc of nonce)
        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);

        // store encrypted data and salt
        self.passwords.insert(service.to_string(), (encrypted_data, salt.to_vec()));
    }

    pub fn get_password(&self, service: String) -> Option<String> {

        // check if the service exists
        if let Some((encrypted_data, _)) = self.passwords.get(&service) {

            // "de-concatenate" the nonce and ciphertext
            let (nonce, ciphertext) = encrypted_data.split_at(12);
            
            // create cipher instance with master key
            let cipher = Aes256Gcm::new(&self.master_key);
            
            // decrypt ciphertext
            let plaintext = cipher.decrypt(nonce.into(), ciphertext)
                .expect("decryption failure!");
            
            // convert plaintext to character format
            return Some(String::from_utf8(plaintext).unwrap())
        } else {
            return None
        }
    }

    pub fn get_username(&self) -> String {
        return self.username.clone()
    }
}
