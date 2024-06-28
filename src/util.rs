use std::io::Write;
use std::path::Path;

use crate::password_manager::PasswordManager;

// ======================================= PROMPTS =======================================

pub fn prompt_main(manager: &PasswordManager) {
    println!();
    let msg = format!("{}'s Password Manager", manager.get_username());
    format_prompt(msg);
    print!("> ");
    std::io::stdout().flush().unwrap();
}

pub fn prompt_signup() -> (String, String, bool) {

    // username loop
    let mut username: String = String::new();
    loop {
        format_prompt("Username".to_string());
        print!("> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut username)
            .expect("\nFailed to read input");
        if valid_username(&username) {
            break;
        }
    }
    let username = username.trim();

    // user is new
    if !Path::new(&format!("users/{}", username)).exists() {
        // password loop
        let mut password: String;
        loop {
            // password prompt message
            format_prompt("Password".to_string());
            print!("> ");
            std::io::stdout().flush().unwrap();

            // disable terminal echoing,
            // then take password input
            password = rpassword::read_password().unwrap();

            // check for character length, specials,
            // uppercase/lowercase, etc
            if valid_password(&password) {
                break;
            }
        }

        return (username.to_string(), password.trim().to_string(), true);
    } 

    // user exists
    else {
        let master_password = prompt_master_password(username.to_string());

        return (
            username.to_string(),
            master_password.trim().to_string(),
            false,
        );
    }
}

// prompt existing user for master password
pub fn prompt_master_password(username: String) -> String {
    let password: String;

    // password prompt message
    format_prompt(format!(
        "Welcome back, {}. Enter your Master Password",
        username
    ));
    print!("> ");
    std::io::stdout().flush().unwrap();

    // disable terminal echoing,
    // then take password input
    password = rpassword::read_password().unwrap();

    return password.trim().to_string();
}

pub fn prompt_service_password(service: String) -> String {
    let password: String;

    // password prompt message
    format_prompt(format!("Enter password for {}", service));
    print!("> ");
    std::io::stdout().flush().unwrap();

    // disable terminal echoing,
    // then take password input
    password = rpassword::read_password().unwrap();
    println!();

    return password.trim().to_string();
}

// =================================== HELPER FUNCtioNS ===================================

fn valid_username(input: &str) -> bool {
    if input.len() < 1 {
        println!("\nInvalid username");
        return false;
    }
    return true;
}

fn valid_password(input: &str) -> bool {
    if input.len() < 1 {
        println!("\nInvalid password");
        return false;
    }
    return true;
}

pub fn format_prompt(msg: String) {
    print!("\n+");
    for _ in 0..msg.len() - 2 {
        print!("=");
    }
    print!("+\n");
    println!("{}", msg);
    print!("+");
    for _ in 0..msg.len() - 2 {
        print!("=");
    }
    print!("+\n");
}

// pub fn print_title_block(title: &str, msg: String) {
//     print!("\n+");
//     for _ in 0..6 {
//         print!("=");
//     }
//     print!(" {title} ");
//     for _ in 0..6 {
//         print!("=");
//     }
//     print!("+\n");

//     let bottom_length = 6 + 1 + title.len() + 1 + 6;
//     println!("{}", msg);
//     print!("+");
//     for _ in 0..bottom_length {
//         print!("=");
//     }
//     print!("+\n");
// }
