use std::io::{self, Write};

fn main() {
    let mut outstr = io::stdout();
    outstr.write_all(b"> ").ok().expect("Cannot write to stdout");
    outstr.flush().ok().expect("Could not flush stdout");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    println!("{}", input);
}
