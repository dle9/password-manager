mod ui;

fn main() {
    let (username, password) = ui::prompt_signup();

    println!("{},{}",username,password);
}
