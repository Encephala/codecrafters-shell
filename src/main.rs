#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    handle_input(&input);
}

fn handle_input(input: &str) {
    // Strip of newline character
    let input = &input[..input.len() - 1];

    match input {
        other => {
            println!("{other}: command not found");
        }
    }
}
