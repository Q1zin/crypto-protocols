pub mod diffie_hellman;
pub mod math;
pub mod poker;
pub mod rsa;
pub mod shamir;

use std::io::{self, Write};

use math::inverse;
use shamir::generate_keys;

/// Спрашивает у пользователя строку
pub fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

/// Спрашивает у пользователя число
pub fn read_number(prompt: &str) -> u64 {
    read_optional(prompt).expect("нужно ввести натуральное число")
}

/// Спрашивает у пользователя число, пустой ввод (Enter) — None
pub fn read_optional(prompt: &str) -> Option<u64> {
    let line = read_line(prompt);
    if line.is_empty() {
        return None;
    }
    Some(line.parse().expect("нужно ввести натуральное число"))
}

/// Пара (c, d) абонента (шифр Шамира, ментальный покер): c вводит пользователь
/// или выбираем случайно, d = c^(-1) mod (p - 1). None — если введённое c не подходит.
pub fn read_keys(name: &str, p: u64) -> Option<(u64, u64)> {
    match read_optional(&format!("c_{name} (Enter — случайное): ")) {
        None => Some(generate_keys(p)),
        Some(c) => match inverse(c, p - 1) {
            Some(d) => Some((c, d)),
            None => {
                println!("c_{name} должно быть взаимно просто с p - 1 = {}", p - 1);
                None
            }
        },
    }
}
