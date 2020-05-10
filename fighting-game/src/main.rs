use std::io::{self, Write};

mod enemy;

fn main() {
    let mut enemy = enemy::Enemy::new();

    loop {
        print_info(&enemy);
        let input = promt();
        if let Some(input) = input {
            match input.as_str().trim() {
                "quit" | "q" => exit(),
                "attack" | "a" => enemy.damage(10),
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

fn print_info(enemy: &enemy::Enemy) {
    println!("Info");
    println!("====");
    println!("Enemy Name: {}", enemy.name);
    println!("Enemy Health: {}", enemy.life);
}

fn exit() {
    println!("Exiting");
    std::process::exit(0);
}
