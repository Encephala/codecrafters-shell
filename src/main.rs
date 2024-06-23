#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_input(&input);
    }
}

fn handle_input(input: &str) {
    // Strip of newline character
    let input = &input[..input.len() - 1];

    let mut words = input.split(' ');

    match words.clone().nth(0).unwrap() {
        "exit" => {
            let code = words.nth(1).unwrap().parse::<i32>().unwrap();

            std::process::exit(code);
        },
        other => {
            println!("{other}: command not found");
        }
    }
}
