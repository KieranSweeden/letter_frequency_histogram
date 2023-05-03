use std::io;

struct UserInput {
    content: String,
}

impl UserInput {
    fn validate(&self) -> bool {
        !&self.content.is_empty()
    }
}

impl UserInput {
    fn prompt() -> UserInput {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        UserInput {
            content: user_input.trim().to_string(),
        }
    }
}

fn run() {
    loop {
        let user_input = UserInput::prompt();
        let is_valid = user_input.validate();
        if !is_valid {
            println!("Provided input was not valid, try again.");
            continue;
        }
    }
}

fn intro() {
    println!("========");
    println!("Welcome!");
    println!("========");
}

fn main() {
    intro();
    run();
}
