use std::io::Write;
use termios::*; // for hiding password input in terminal

use crate::password_manager::PasswordManager;
// ======================================= PROMPTS =======================================

pub fn prompt_signup() -> (String, String) {
    
    // username loop
    let mut username: String = String::new();
    loop {
        format_prompt("Username".to_string());
        print!("> "); std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut username).expect("\nFailed to read input");
        let username = username.trim();
        
        if valid_username(username) {   
            break;
        }   
    }

    // TODO: check for existing user
    
    // password loop
    let mut password: String;
    loop {
        // password prompt message
        format_prompt("Password".to_string());
        print!("> "); std::io::stdout().flush().unwrap();
        
        // disable terminal echoing, 
        // then take password input
        password = read_password().unwrap();
        
        // check for character length, specials, 
        // uppercase/lowercase, etc
        if valid_password(&password) {   
            break;
        }   
    }
    
    return (username.trim().to_string(), password.trim().to_string());
}

// TODO: check for existing user, if exists
// use this function instead of prompting for
// a valid password during prompt_signup
pub fn prompt_master_password() -> String {
    let password: String;

    // password prompt message
    format_prompt("Enter your Master Password".to_string());
    print!("> "); std::io::stdout().flush().unwrap();
    
    // disable terminal echoing, 
    // then take password input
    password = read_password().unwrap();
        
    return password.trim().to_string();
}

pub fn prompt_service_password(service: String) -> String {
    let password: String;

    // password prompt message
    format_prompt(format!("Enter password for {}", service));
    print!("> "); std::io::stdout().flush().unwrap();
    
    // disable terminal echoing, 
    // then take password input
    password = read_password().unwrap();
    println!();
        
    return password.trim().to_string();
}

pub fn prompt_main(manager: &PasswordManager) {  
    let msg = format!("{}'s Password Manager", manager.get_username());
    format_prompt(msg);
    print!("> "); std::io::stdout().flush().unwrap();
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

fn read_password() -> std::io::Result<String> {
    let stdin = 0;
    let termios = Termios::from_fd(stdin)?;
    let mut new_termios = termios.clone();
    
    // disable echo
    new_termios.c_lflag &= !(ECHO | ICANON);
    tcsetattr(stdin, TCSANOW, &new_termios)?;
    
    let mut password = String::new();
    std::io::stdin().read_line(&mut password)?;
    
    // restore terminal
    tcsetattr(stdin, TCSANOW, &termios)?;
    
    // remove the trailing newline
    password.trim().to_string();
    
    Ok(password)
}

pub fn format_prompt(msg: String) {
    print!("\n+"); for _ in 0..msg.len()-2 { print!("="); } print!("+\n");
    println!("{}", msg);
    print!("+"); for _ in 0..msg.len()-2 { print!("="); } print!("+\n");
}

pub fn print_title_block(title: &str, msg: String) {
    print!("\n+"); for _ in 0..6 { print!("="); } 
    print!(" {title} ");
    for _ in 0..6 { print!("="); } print!("+\n");
    
    let bottom_length = 6 + 1 + title.len() + 1 + 6;
    println!("{}", msg);
    print!("+"); for _ in 0..bottom_length { print!("="); } print!("+\n"); 
}