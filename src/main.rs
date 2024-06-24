// // cryptography
// use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Key};

// // prompts, displays, etc
// mod ui;

// fn generate_key_from_password(password: &str) -> Key<Aes256Gcm> {
//     // Pad or truncate the password to exactly 32 bytes
//     let mut key_bytes = [0u8; 32];
//     let password_bytes = password.as_bytes();
//     let copy_len = password_bytes.len().min(32);
//     key_bytes[..copy_len].copy_from_slice(&password_bytes[..copy_len]);

//     Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
// }

// fn main() {
//     // get the user
//     let (username, password) = ui::prompt_signup();

//     // generate key based on a string
//     let key: Key<Aes256Gcm> = generate_key_from_password(&password);

//     // create aes instance
//     let cipher = Aes256Gcm::new(&key);

//     // create message
//     let message = b"Hello, World!";

//     // create random nonce
//     let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

//     // encrypt message
//     let ciphertext = cipher
//         .encrypt(&nonce, message.as_ref())
//         .expect("encryption failure!");

//     // decrypt message
//     let plaintext = cipher
//         .decrypt(&nonce, ciphertext.as_ref())
//         .expect("decryption failure!");

//     // result
//     println!("\n\nusername:{}\npassword:{}", username, password);
//     println!("Encrypted (bytes): {:?}", &ciphertext);
//     println!("Encrypted (text): {:?}", String::from_utf8_lossy(&ciphertext));
//     println!("Decrypted: {:?}", String::from_utf8_lossy(&plaintext));
// }

mod password_manager; use password_manager::PasswordManager;
mod ui;

fn main() {
    let (_username, password) = ui::prompt_signup();

    let mut pass_man = PasswordManager::new(&password);
    
    pass_man.add_password("example.com", "password123");
    pass_man.add_password("example.org", "password123");

    println!("example.com password: {:?}", pass_man.get_password("example.com"));
    println!("example.org password: {:?}", pass_man.get_password("example.org"));
}