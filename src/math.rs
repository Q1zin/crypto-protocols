/// Обобщённый алгоритм Евклида (алгоритм 2.2).
///
/// Возвращает (gcd, x, y), такие что a*x + b*y = gcd(a, b).
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
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

/// Инверсия: d = c^(-1) mod m, то есть c*d mod m = 1 (определение 2.6).
///
/// Обобщённый Евклид для пары (m, c) даёт m*x + c*y = 1, тогда d = y.
/// Если y < 0, прибавляем m. Если gcd(c, m) != 1, инверсии нет — None.
pub fn inverse(c: u64, m: u64) -> Option<u64> {
    let (g, _x, y) = ext_gcd(m as i64, (c % m) as i64);
    if g != 1 {
        return None;
    }

    let mut d = y;
    if d < 0 {
        d += m as i64;
    }
    Some(d as u64)
}
