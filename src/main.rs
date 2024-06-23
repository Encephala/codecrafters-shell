use std::collections::HashMap;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn main() {
    let env: HashMap<_, _> = std::env::vars().collect();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_input(&input, &env);
    }
}

fn handle_input(input: &str, env: &HashMap<String, String>) {
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
            let result = type_builtin(input, env);

            println!("{result}");
        }
        other => {
            let found_executable = find_executable(env.get("PATH").or(Some(&"".to_string())).unwrap().split(':'), other);

            if found_executable.is_none() {
                println!("{other}: command not found");

                return ();
            }

            let executable = found_executable.unwrap();

            let args = input.split(' ').skip(1).collect();

            execute(&executable, &args);
        }
    }
}

fn type_builtin(input: &str, env: &HashMap<String, String>) -> String {
    let known_commands = [
        ("echo", "shell builtin"),
        ("exit", "shell builtin"),
        ("type", "shell builtin"),
    ];
    let definitions = HashMap::from(known_commands);

    let command = input.split(' ').nth(1).unwrap();

    match definitions.get(command) {
        Some(definition) => return format!("{command} is a {definition}"),
        None => {
            let path = env.get("PATH");

            if path.is_none() {
                return format!("{command}: not found");
            }

            let found_executable = find_executable(path.unwrap().split(':'), command);

            if found_executable.is_none() {
                return format!("{command}: not found");
            }

            let executable = found_executable.unwrap();

            let file_is_executable = {
                let permissions = executable.metadata().unwrap().permissions();

                permissions.mode() & 0o001 != 0
            };

            if !file_is_executable {
                return format!("{command}: not found");
            }

            return format!("{command} is {}", executable.to_str().unwrap());
        }
    };
}

fn find_executable<'a>(path: impl Iterator<Item = &'a str>, name: &str) -> Option<std::path::PathBuf> {
    for directory in path {
        let full_path = std::path::PathBuf::from(directory).join(name);
        if full_path.exists() {
            return Some(full_path);
        }
    }

    return None;
}

fn execute(executable: &Path, args: &Vec<&str>) {
    Command::new(executable)
        .args(args)
        .status()
        .unwrap();
}
