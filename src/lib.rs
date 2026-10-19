pub mod diffie_hellman;
pub mod math;

use std::io::{self, Write};

/// Спрашивает у пользователя число.
pub fn read_number(prompt: &str) -> u64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().expect("нужно ввести натуральное число")
}
