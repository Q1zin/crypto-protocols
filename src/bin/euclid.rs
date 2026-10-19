//! Обобщённый алгоритм Евклида: a*x + b*y = gcd(a, b).
//! Запуск: cargo run --bin euclid

use crypto_protocols::math::ext_gcd;
use crypto_protocols::read_number;

fn main() {
    let a = read_number("a = ") as i64;
    let b = read_number("b = ") as i64;

    let (g, x, y) = ext_gcd(a, b);

    println!("gcd({a}, {b}) = {g}, x = {x}, y = {y}");
    println!("Проверка: {a}*({x}) + {b}*({y}) = {}", a * x + b * y);
}
