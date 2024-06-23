use std::collections::HashMap;
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
    if input.len() == 0 {
        println!();
        return ();
    }

    let input = &input[..input.len() - 1];

    let mut words = input.split(' ');

    match words.clone().nth(0).unwrap() {
        "exit" => {
            let code = words.nth(1).map(|number| number.parse::<i32>().unwrap());

            match code {
                Some(code) => std::process::exit(code),
                None => std::process::exit(0)
            }
        },
        "echo" => {
            println!("{}", &input[5..]);
        },
        "type" => {
            type_builtin(input)
        }
        other => {
            println!("{other}: command not found");
        }
    }
}

fn type_builtin(input: &str) {
    let known_commands = [
        ("echo", "shell builtin"),
        ("exit", "shell builtin"),
        ("type", "shell builtin"),
    ];
    let definitions = HashMap::from(known_commands);

    let command = input.split(' ').nth(1).unwrap();

    let output = match definitions.get(command) {
        Some(definition) => format!("{command} is a {definition}"),
        None => format!("{command}: not found"),
    };

    println!("{}", output);
}
