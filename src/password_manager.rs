// cryptography
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key,
};
use pbkdf2::pbkdf2_hmac;
use rand_core::{OsRng, RngCore};
use sha2::Sha256;

// file i/o
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

use crate::util;

// TODO: add persistent master key
pub struct PasswordManager {
    username: String,
    master_key: Key<Aes256Gcm>,
}

impl PasswordManager {
    pub fn new(username: String, master_password: String, new_user: bool) -> Self {
        // make user file for new user
        if new_user {
            File::create(&format!("data/{}", username.clone())).expect("Failed to create file");
        }

        // derive key from password input using PBKDF2
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(master_password.as_bytes(), b"", 10000, &mut key);

        // create the key
        let master_key = Key::<Aes256Gcm>::from_slice(&key).clone();

        return PasswordManager {
            username,
            master_key,
        };
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
        let ciphertext = cipher
            .encrypt(&nonce, password.as_bytes())
            .expect("encryption failure!");

        // concatenate nonce+ciphertext (same plaintext will map to diff ciphertext bc of nonce)
        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);

        // concatenate service, nonce, and ciphertext
        let mut entry = service.into_bytes().to_vec();
        entry.push(0); // separator for service and password
        entry.extend_from_slice(hex::encode(&encrypted_data).as_bytes());

        // append entry to file
        let mut file = OpenOptions::new()
            .append(true)
            .open(&format!("data/{}", self.get_username()))
            .expect("Failed to open file");

        file.write_all(&entry).expect("Failed to write entry");
        file.write_all(&[b'\n']).expect("Failed to write newline");
    }

    pub fn get_password(&self, service: String) -> Option<String> {

        // open the file for reading
        let file = File::open(&format!("data/{}", self.get_username())).expect("Failed to open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");

            // split service and password by the null byte 
            // that was appended in add_password()
            let parts: Vec<&str> = line.splitn(2, '\0').collect();
            
            if parts[0] == service {
                match hex::decode(parts[1]) {
                    Ok(data) => {
                        let encrypted_data = data;

                        // nonce is 12 elements
                        let (nonce, ciphertext) = encrypted_data.split_at(12);

                        // create cipher instance with master key
                        let cipher = Aes256Gcm::new(&self.master_key);

                        // decrypt it
                        let plaintext = cipher.decrypt(nonce.into(), ciphertext).expect(
                            "\nDecryption failure. Did you type the correct Master Password?\n",
                        );

                        return Some(String::from_utf8(plaintext).unwrap());
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }

        return None;
    }

    pub fn get_username(&self) -> String {
        return self.username.clone();
    }
}
