pub mod diffie_hellman;
pub mod math;
pub mod shamir;

use std::io::{self, Write};

/// Спрашивает у пользователя число
pub fn read_number(prompt: &str) -> u64 {
    read_optional(prompt).expect("нужно ввести натуральное число")
}

/// Спрашивает у пользователя число, пустой ввод (Enter) — None
pub fn read_optional(prompt: &str) -> Option<u64> {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    Some(line.parse().expect("нужно ввести натуральное число"))
}
