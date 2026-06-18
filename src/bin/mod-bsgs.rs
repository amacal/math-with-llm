use std::collections::HashMap;

fn main() {
    let p = 4_294_967_291u64;
    let g = find_gen(p).unwrap();
    println!("Generator of {}: {}", p, g);

    for i in 1..std::cmp::min(p, 64) {
        println!("BGSG({}, {}, {}): {:?}", g, p, i, bgsg(g, p, i));
    }
}

fn gcd(mut a: u64, mut b: u64) -> Option<(u64, (i64, i64))> {
    // edge case: gcd(0, 0) is undefined
    if a == 0 && b == 0 {
        return None;
    }

    // seed values
    let mut q;
    let (mut xa, mut ya): (i64, i64) = (1, 0);
    let (mut xb, mut yb): (i64, i64) = (0, 1);

    // stop when a is already the gcd
    while b != 0 {
        let (ta, tx, ty) = (b, xb, yb);
        (b, q) = (a % b, a / b);

        // if the cast is unsuccessful, it means the q doesn't fit i64, implying that b is 0
        // and we have found the gcd, skipping the update of the linear coefficients
        if let Ok(q) = i64::try_from(q) {
            (xb, yb) = (xa - q * xb, ya - q * yb);
        }

        (a, xa, ya) = (ta, tx, ty);
    }

    // determined gcd and its linear coefficients
    Some((a, (xa, ya)))
}

fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    return match gcd(a, m) {
        Some((1, (x, _))) => match (u64::try_from(x), u64::try_from(-x)) {
            (Ok(x), _) => Some(x),
            (_, Ok(x)) => Some(m - x),
            _ => None,
        },
        _ => None,
    };
}

fn mod_exp(base: u64, mut exp: u64, modulus: u64) -> Option<u64> {
    if modulus == 0 {
        return None;
    }

    let mut power: u64 = base % modulus;
    let mut result: u64 = 1;
    let mut tmp: u128;

    while exp > 0 {
        if exp & 0x01 == 0x01 {
            tmp = result as u128 * power as u128;
            result = (tmp % modulus as u128) as u64;
        }

        tmp = power as u128 * power as u128;
        power = (tmp % modulus as u128) as u64;

        exp >>= 1;
    }

    return Some(result);
}

#[derive(Debug, PartialEq)]
enum MillRabinResult {
    IsComposite,
    MaybePrime,
}

fn miller_rabin(a: u64, n: u64) -> Option<MillRabinResult> {
    if a >= n || a == 0 || n == 0 {
        return None;
    }

    let mut d: u64 = n - 1;
    let mut r: u64 = 0;

    // decompose a to 2^r * d
    while d > 0 && d & 0x01 == 0x00 {
        d >>= 0x01;
        r += 1;
    }

    // find first a^d mod n
    let mut exp: u64 = mod_exp(a, d, n)?;

    // if a^d mod n is 1, then n is maybe prime
    if exp == 1 {
        return Some(MillRabinResult::MaybePrime);
    }

    while r > 0 {
        // if a^(2^r * d) mod n is -1, then n is maybe prime
        if exp == n - 1 {
            return Some(MillRabinResult::MaybePrime);
        }

        r = r - 1;
        exp = ((exp as u128) * (exp as u128) % (n as u128)) as u64;
    }

    // it must be composite if we reach here
    return Some(MillRabinResult::IsComposite);
}

fn miller_rabin_test(n: u64, k: u64) -> Option<bool> {
    if n < 2 {
        return None;
    }

    for a in 2..std::cmp::min(n, k) {
        match miller_rabin(a, n)? {
            MillRabinResult::IsComposite => return Some(false),
            MillRabinResult::MaybePrime => (),
        }
    }

    return Some(true);
}

fn find_gen(p: u64) -> Option<u64> {
    if miller_rabin_test(p, 20)? == false {
        return None;
    }

    for g in 2..p {
        let exp1 = mod_exp(g, 2, p)?;
        let exp2 = mod_exp(g, (p - 1) / 2, p)?;

        if exp1 != 1 && exp2 != 1 {
            return Some(g);
        }
    }

    return None;
}

fn bgsg(g: u64, p: u64, h: u64) -> Option<u64> {
    if miller_rabin_test(p, 20)? == false {
        return None;
    }

    let m = (p as f64).sqrt().ceil() as u64;
    let inv = mod_inverse(g, p)?;
    let mut map = HashMap::<u64, u64>::new();

    for i in 0..m {
        let exp = mod_exp(inv, i, p)?;
        map.insert((h as u128 * exp as u128 % (p as u128)) as u64, i);
    }

    for i in 0..m {
        let exp = mod_exp(g, i * m, p)?;

        if let Some(&j) = map.get(&exp) {
            return Some(((i * m) as u128 + j as u128) as u64);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_gen() {
        assert_eq!(find_gen(11), Some(2));
        assert_eq!(find_gen(65537), Some(3));
    }

    #[test]
    fn test_bgsg_11() {
        assert_eq!(bgsg(2, 11, 0), None);
        assert_eq!(bgsg(2, 11, 1), Some(0));
        assert_eq!(bgsg(2, 11, 2), Some(1));
        assert_eq!(bgsg(2, 11, 3), Some(8));
        assert_eq!(bgsg(2, 11, 4), Some(2));
        assert_eq!(bgsg(2, 11, 5), Some(4));
        assert_eq!(bgsg(2, 11, 6), Some(9));
        assert_eq!(bgsg(2, 11, 7), Some(7));
        assert_eq!(bgsg(2, 11, 8), Some(3));
        assert_eq!(bgsg(2, 11, 9), Some(6));
        assert_eq!(bgsg(2, 11, 10), Some(5));
    }

    #[test]
    fn test_bgsg_roundtrip() {
        for i in 1..11 {
            let x = bgsg(2, 11, i).unwrap();
            let y = mod_exp(2, x, 11).unwrap();

            assert_eq!(i, y);
        }
    }

    #[test]
    fn test_bgsg_zero() {
        let p = 4_294_967_291u64;
        let g = find_gen(p).unwrap();

        assert_eq!(bgsg(g, p, 0), None);
    }

    #[test]
    fn test_bgsg_not_prime() {
        let p = 4_294_967_289u64;
        assert_eq!(find_gen(p), None);
    }

    #[test]
    fn test_bgsg_u32_max() {
        let p = 4_294_967_291u64;
        let g = find_gen(p).unwrap();

        assert_eq!(bgsg(g, p, 1048576), Some(20));
    }
}
