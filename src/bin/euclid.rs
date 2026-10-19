//! Обобщённый алгоритм Евклида: a*x + b*y = gcd(a, b).
//! Запуск: cargo run --bin euclid

use std::io::{self, Write};

fn read_number(prompt: &str) -> u64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().parse().expect("нужно ввести натуральное число")
}

/// Обобщённый алгоритм Евклида (алгоритм 2.2).
///
/// Возвращает (gcd, x, y), такие что a*x + b*y = gcd(a, b).
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut u = (a, 1, 0);
    let mut v = (b, 0, 1);
    while v.0 != 0 {
        let q = u.0 / v.0;
        let t = (u.0 % v.0, u.1 - q * v.1, u.2 - q * v.2);
        u = v;
        v = t;
    }
    u
}

fn main() {
    let a = read_number("a = ") as i64;
    let b = read_number("b = ") as i64;

    let (g, x, y) = ext_gcd(a, b);

    println!("gcd({a}, {b}) = {g}, x = {x}, y = {y}");
    println!("Проверка: {a}*({x}) + {b}*({y}) = {}", a * x + b * y);
}
