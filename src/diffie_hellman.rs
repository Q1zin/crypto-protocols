//! Система Диффи–Хеллмана (2.20).
//!
//! Открытые параметры: простое p и число g
//! Абонент выбирает секретное X и публикует Y = g^X mod p
//! A и B получают общий ключ Z_AB = Y_B^X_A mod p = Y_A^X_B mod p = Z_BA

use crate::math::{is_prime, pow_mod, random_range};

/// Проверка параметров: p = 2q + 1, где p и q простые,
/// 1 < g < p - 1 и g^q mod p != 1
pub fn check_params(p: u64, g: u64) -> bool {
    let q = (p - 1) / 2;
    is_prime(p) && is_prime(q) && 1 < g && g < p - 1 && pow_mod(g, q, p) != 1
}

/// Случайный секретный ключ X из [2, p - 2]
pub fn secret_key(p: u64) -> u64 {
    random_range(2, p - 2)
}

/// Открытый ключ: Y = g^X mod p
pub fn public_key(g: u64, x: u64, p: u64) -> u64 {
    pow_mod(g, x, p)
}

/// Общий ключ: Z = (Y другого абонента)^X mod p
pub fn shared_key(y_other: u64, x: u64, p: u64) -> u64 {
    pow_mod(y_other, x, p)
}
