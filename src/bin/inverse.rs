//! Инверсия: d = c^(-1) mod m.
//! Запуск: cargo run --bin inverse

use crypto_protocols::math::inverse;
use crypto_protocols::read_number;

fn main() {
    let c = read_number("c = ");
    let m = read_number("m = ");

    match inverse(c, m) {
        Some(d) => {
            println!("{c}^(-1) mod {m} = {d}");
            println!("Проверка: {c}*{d} mod {m} = {}", c * d % m);
        }
        None => println!("Инверсии нет: gcd({c}, {m}) != 1"),
    }
}
