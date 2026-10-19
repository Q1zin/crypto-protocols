//! Шифр Шамира (2.21).
//!
//! Открытый параметр — простое p. У каждого абонента секретная пара (c, d),
//! c*d mod (p - 1) = 1. Сообщение m < p передаётся за 4 шага.

use crate::math::{inverse, pow_mod, random_range};

/// Секретные ключи абонента (c, d): c*d mod (p - 1) = 1.
///
/// c — случайное, взаимно простое с p - 1; d = c^(-1) mod (p - 1).
/// Если c и p - 1 не взаимно просты, inverse вернёт None — берём другое c.
/// c = 1 и c = p - 2 не берём: тогда x1 = m или x1 = m^(-1), и m легко узнать.
pub fn generate_keys(p: u64) -> (u64, u64) {
    loop {
        let c = random_range(3, p - 3);
        if let Some(d) = inverse(c, p - 1) {
            return (c, d);
        }
    }
}

/// Передача сообщения m от A к B. Возвращает (x1, x2, x3, x4); должно быть x4 = m.
pub fn transmit(m: u64, p: u64, c_a: u64, d_a: u64, c_b: u64, d_b: u64) -> (u64, u64, u64, u64) {
    let x1 = pow_mod(m, c_a, p); // шаг 1: A -> B
    let x2 = pow_mod(x1, c_b, p); // шаг 2: B -> A
    let x3 = pow_mod(x2, d_a, p); // шаг 3: A -> B
    let x4 = pow_mod(x3, d_b, p); // шаг 4: у B
    (x1, x2, x3, x4)
}
