//! Лабораторная 2.21 — шифр Шамира.
//! Запуск: cargo run --bin shamir

use crypto_protocols::read_number;
use crypto_protocols::shamir::{generate_keys, transmit};

fn main() {
    let p = 30803;
    let m = read_number(&format!("Сообщение m (меньше {p}): "));
    if m >= p {
        println!("Сообщение должно быть меньше p = {p}");
        return;
    }

    // Каждый абонент выбирает свою пару c, d.
    let (c_a, d_a) = generate_keys(p);
    let (c_b, d_b) = generate_keys(p);
    println!("A: c_A = {c_a}, d_A = {d_a}");
    println!("B: c_B = {c_b}, d_B = {d_b}");

    let (x1, x2, x3, x4) = transmit(m, p, c_a, d_a, c_b, d_b);
    println!("Шаг 1. A -> B: x1 = {m}^{c_a} mod {p} = {x1}");
    println!("Шаг 2. B -> A: x2 = {x1}^{c_b} mod {p} = {x2}");
    println!("Шаг 3. A -> B: x3 = {x2}^{d_a} mod {p} = {x3}");
    println!("Шаг 4. B:      x4 = {x3}^{d_b} mod {p} = {x4}");

    if x4 == m {
        println!("B получил сообщение m = {x4}");
    } else {
        println!("Ошибка: x4 != m");
    }
}
