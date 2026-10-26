//! Лабораторная 2.23 — шифр RSA.
//! Запуск: cargo run --bin rsa

use crypto_protocols::math::is_prime;
use crypto_protocols::rsa::{generate_keys, transmit};
use crypto_protocols::{read_line, read_number, read_optional};

fn main() {
    // Каждый абонент выбирает P, Q и d. Открытые N и d, секретное c.
    let Some((n_a, d_a, c_a)) = read_keys("A", 131, 227) else {
        return;
    };
    let Some((n_b, d_b, c_b)) = read_keys("B", 113, 281) else {
        return;
    };
    println!("A: открытые N_A = {n_a}, d_A = {d_a}; секретное c_A = {c_a}");
    println!("B: открытые N_B = {n_b}, d_B = {d_b}; секретное c_B = {c_b}");

    // Латинская или русская буква — на русской раскладке «А» и «В» тоже подходят.
    let (name, n, d, c) = match read_line("Кому отправить, A или B (Enter — B): ")
        .to_uppercase()
        .as_str()
    {
        "A" | "А" => ("A", n_a, d_a, c_a),
        "" | "B" | "В" => ("B", n_b, d_b, c_b),
        _ => {
            println!("Получатель — A или B");
            return;
        }
    };

    let m = read_number(&format!("Сообщение m (меньше N_{name} = {n}): "));
    if m >= n {
        println!("Сообщение должно быть меньше N_{name} = {n}");
        return;
    }

    let (e, m2) = transmit(m, n, d, c);
    println!("Шаг 1. Отправитель -> {name}: e = {m}^{d} mod {n} = {e}");
    println!("Шаг 2. {name}:              m' = {e}^{c} mod {n} = {m2}");

    if m2 == m {
        println!("{name} получил сообщение m = {m2}");
    } else {
        println!("Ошибка: m' != m");
    }
}

/// Ключи абонента (N, d, c): P, Q и d вводит пользователь (Enter — значения из 2.23),
/// N = P*Q, c = d^(-1) mod (P - 1)(Q - 1). None — если параметры не подходят.
fn read_keys(name: &str, p: u64, q: u64) -> Option<(u64, u64, u64)> {
    let p = read_optional(&format!("P_{name} (Enter — {p}): ")).unwrap_or(p);
    let q = read_optional(&format!("Q_{name} (Enter — {q}): ")).unwrap_or(q);
    if p == q || !is_prime(p) || !is_prime(q) {
        println!("P_{name} и Q_{name} должны быть разными простыми числами");
        return None;
    }

    let d = read_optional(&format!("d_{name} (Enter — 3): ")).unwrap_or(3);
    match generate_keys(p, q, d) {
        Some((n, c)) => Some((n, d, c)),
        None => {
            println!(
                "d_{name} должно быть взаимно просто с (P - 1)(Q - 1) = {}",
                (p - 1) * (q - 1)
            );
            None
        }
    }
}
