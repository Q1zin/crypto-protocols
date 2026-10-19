//! Лабораторная 2.20 — система Диффи–Хеллмана.
//! Запуск: cargo run --bin dh

use crypto_protocols::diffie_hellman::{check_params, public_key, secret_key, shared_key};

fn main() {
    let p = 30803;
    let g = 2;
    println!(
        "p = {p}, g = {g}, параметры подходят: {}",
        check_params(p, g)
    );

    // Каждый абонент выбирает секретный X и вычисляет открытый Y.
    let x_a = secret_key(p);
    let x_b = secret_key(p);
    let y_a = public_key(g, x_a, p);
    let y_b = public_key(g, x_b, p);
    println!("A: секретный X_A = {x_a}, открытый Y_A = {y_a}");
    println!("B: секретный X_B = {x_b}, открытый Y_B = {y_b}");

    // Каждый вычисляет общий ключ по открытому ключу другого.
    let z_ab = shared_key(y_b, x_a, p); // считает A
    let z_ba = shared_key(y_a, x_b, p); // считает B
    println!("A вычислил Z_AB = {y_b}^{x_a} mod {p} = {z_ab}");
    println!("B вычислил Z_BA = {y_a}^{x_b} mod {p} = {z_ba}");

    if z_ab == z_ba {
        println!("Ключи совпали — общий секретный ключ {z_ab}");
    } else {
        println!("Ошибка: ключи не совпали");
    }
}
