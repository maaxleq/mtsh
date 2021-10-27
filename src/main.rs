use std::io::{Write};

fn main() {
    let mut input = String::new();
    let mut splitted: std::str::SplitWhitespace;
    let mut count: usize;

    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        splitted = input.split_whitespace();
        count = splitted.clone().count();

        if count == 0 {
            eprintln!("Could not parse command");
        }
        else {
            let mut command = std::process::Command::new(splitted.next().unwrap());

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
}
