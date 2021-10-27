use std::io::{Write};

fn process_command(cmd_string: &String) -> bool {
    let mut splitted: std::str::SplitWhitespace;
    let count: usize;
    let first_word: &str;
    let will_continue = true;

    splitted = cmd_string.split_whitespace();
    count = splitted.clone().count();

    if count == 0 {
        eprintln!("Could not parse command");
        return will_continue;
    }

    first_word = splitted.next().unwrap();

    match first_word {
        "cd" => {
            match splitted.next() {
                Some(path) => {
                    match std::env::set_current_dir(path) {
                        Ok(_) => (),
                        Err(_) => eprintln!("Invalid path")
                    }
                },
                None => eprintln!("No path provided")
            }
        }
        _ => {
            let mut command = std::process::Command::new(first_word);

            for arg in splitted {
                command.arg(arg);
            }
        
            match command.spawn() {
                Ok(mut child) => {
                    child.wait().unwrap();
                },
                Err(_) => {
                    eprintln!("Invalid command");
                }
            }
        }
    }

    return will_continue;
}

fn main() {
    let mut input = String::new();

    loop {
        print!("{}|>", std::env::current_dir().unwrap().display());
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        if ! process_command(&input) {
            break;
        }
    }
}
