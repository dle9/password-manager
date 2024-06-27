use clap::{Parser, Subcommand};

mod password_manager; use password_manager::PasswordManager;
mod util;

fn main() {
    let (username, password) = util::prompt_signup();
    let mut manager = PasswordManager::new(username, password);
    
    // main interactive loop
    loop {
        util::prompt_main(&manager);

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        // automatically include initial arg
        let input = format!("{} {}", "> ", input);

        // split the input into args
        let args = shlex::split(&input).ok_or("error: Invalid quoting").unwrap();

        // parse the input
        let cli = Cli::try_parse_from(args.iter());

        // check if parsing was successful
        match cli {
            Ok(cli) => {
                // handle the input
                match cli.commands {
                    Commands::Add{service} => {
                        manager.add_password(service);
                    },
                    Commands::Get{service} => {
                        match manager.get_password(service.clone()) {
                            Some(password) => println!("{}: {:?}", service.clone(), password),
                            None => println!("\n{} doesn't exist", service.clone()),
                        }
                    },

                    // TODO
                    Commands::List{} => println!("list"),

                    Commands::Exit{} => { println!("\nGoodbye"); break; },
                }
            }
            Err(e) => println!("{}", e),
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about = "add an entry        ::", visible_alias = "a")]
    Add{service: String},
    
    #[clap(about = "get an entry        ::", visible_alias = "g")]
    Get{service: String},

    #[clap(about = "list all entries    ::", visible_alias = "l")]
    List{},

    #[clap(about = "exit the program", visible_aliases = &["q", "e"])]
    Exit{},
}