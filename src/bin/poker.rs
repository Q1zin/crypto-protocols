//! Лабораторная 5.3 — ментальный покер.
//! Запуск: cargo run --bin poker

use crypto_protocols::math::{is_prime, pow_mod, random_range};
use crypto_protocols::poker::{card, encode_card, shuffle};
use crypto_protocols::{read_keys, read_optional};

fn main() {
    let p = read_optional("p (Enter — 30803): ").unwrap_or(30803);
    if p < 11 || !is_prime(p) {
        println!("p должно быть простым и не меньше 11");
        return;
    }

    // (1) Каждый игрок выбирает свою пару c, d.
    let Some((c_a, d_a)) = read_keys("A", p) else {
        return;
    };
    let Some((c_b, d_b)) = read_keys("B", p) else {
        return;
    };
    println!("A: c_A = {c_a}, d_A = {d_a}");
    println!("B: c_B = {c_b}, d_B = {d_b}");

    // Числа карт R = r||код, какое число какой карте соответствует — знают оба.
    let r: Vec<u64> = (0..3).map(|code| encode_card(code, p)).collect();
    for &r in &r {
        println!("   R = {r} = {r:b}₂ — {}", card(r));
    }

    // (2) A шифрует карты своим c_A, перемешивает и отправляет B.
    let mut x: Vec<u64> = r.iter().map(|&r| pow_mod(r, c_a, p)).collect();
    shuffle(&mut x);
    println!("(2) A -> B: x = {x:?} (x_i = R_i^c_A mod p, перемешаны)");

    // B выбирает одно из x и отправляет A — это карта A, она снимает свою степень.
    let x_a = x.remove(random_range(0, 2) as usize);
    let card_a = pow_mod(x_a, d_a, p);
    println!(
        "    B -> A: {x_a}, A: {x_a}^d_A mod p = {card_a} — карта A: {}",
        card(card_a)
    );

    // (3) B шифрует оставшиеся два своим c_B, перемешивает и отправляет A.
    let mut y: Vec<u64> = x.iter().map(|&x| pow_mod(x, c_b, p)).collect();
    shuffle(&mut y);
    println!("(3) B -> A: y = {y:?} (y_j = x_j^c_B mod p, перемешаны)");

    // A выбирает одно из y — это будет карта B, второе остаётся в прикупе.
    let y_b = y.remove(random_range(0, 1) as usize);
    let z = pow_mod(y_b, d_a, p);
    println!("(4) A -> B: z = {y_b}^d_A mod p = {z}");
    let w = pow_mod(z, d_b, p);
    println!("(5) B: w = {z}^d_B mod p = {w} — карта B: {}", card(w));

    // Прикуп = R^(c_A c_B) mod p, его не знает никто. Для проверки снимаем обе степени.
    let left = pow_mod(pow_mod(y[0], d_a, p), d_b, p);
    println!(
        "В прикупе {} — игроки его не знают (это {})",
        y[0],
        card(left)
    );

    let mut dealt = vec![card_a, w, left];
    dealt.sort();
    let mut all = r;
    all.sort();
    if dealt == all {
        println!("Карты розданы правильно: у A, у B и в прикупе разные карты");
    } else {
        println!("Ошибка: розданные карты не совпали с R");
    }
}
