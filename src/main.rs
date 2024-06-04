use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.trim().to_string()
}

fn rot13(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let rotated = (((c as u8) - offset + 13) % 26) + offset;
            rotated as char
        } else {
            c
        }
    }).collect()
}
use std::process::Command;

fn copy(s: &str) {
    let mut command = Command::new("wl-copy");
    command.arg(s);

    match command.spawn() {
        Ok(child) => {
            println!("\x1b[32mCopied via wl-copy with PID: {}\x1b[0m", child.id());
        }
        Err(e) => {
            eprintln!("\x1b[31mFailed to execute command: {}\x1b[0m", e);
        }
    }
}

fn main() {
    print!("\x1b[34m");
    let str = get_user_input("input: ");
    let rot = rot13(&str);
    println!("\x1b[36mrot13: {}", rot);
    copy(&rot);
}
