use std::io::{self, Write};

pub fn intro() {
    println!();
    println!("=== Instructions ===");
    println!("Type A, B, C, or D (Non-case sensitive) to answer");
    println!("Type exit to leave the game");
    println!("Type help to show this again");
    println!();
}

pub fn prompt(symbol: &str) -> String {
    let mut line: String = String::new();

    print!("{}", symbol);

    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut line)
        .expect("Invalid line argument");

    line.trim().to_string()
}
