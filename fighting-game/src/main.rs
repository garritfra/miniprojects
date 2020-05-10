use std::io::{self, Write};

fn main() {
    loop {
        let input = promt();
        if let Some(input) = input {
            match input.as_str().trim() {
                "quit" => exit(),
                _ => println!("Cannot evaluate input"),
            }
        }
    }
}

fn promt() -> Option<String> {
    let mut outstr = io::stdout();
    outstr.write(b"> ").ok().expect("Cannot write to stdout");

    outstr.flush().ok().expect("Could not flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    if !input.is_empty() {
        Some(input)
    } else {
        None
    }
}

fn exit() {
    println!("Exiting");
    std::process::exit(0);
}
