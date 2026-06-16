fn main() {
    println!("Modular inverse of 17 and 19 is: {:?}", mod_inverse(17, 19));
}

fn gcd(mut a: u64, mut b: u64) -> Option<u64> {
    if a == 0 && b == 0 {
        return None;
    }

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    Some(a)
}

fn totient(mut n: u64) -> u64 {
    let mut result: u64 = 1;
    let mut p = 2;

    while p * p <= n {
        let mut k: u64 = 0;
        let mut t: u64 = 1;

        while n % p == 0 {
            n /= p;
            k += 1;
        }

        while k > 0 {
            t *= p;
            k -= 1;
        }

        if t > 1 {
            result *= t - t / p;
        }

        p += 1;
    }

    return result * if n > 1 { n - 1 } else { 1 };
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

fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    match (m, gcd(a, m)) {
        (0, _) | (1, _) => None,
        (_, Some(1)) => mod_exp(a, totient(m) - 1, m),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(15, 22), Some(3));
        assert_eq!(mod_inverse(22, 15), Some(13));

        assert_eq!(mod_inverse(17, 19), Some(9));
        assert_eq!(mod_inverse(19, 17), Some(9));
    }

    #[test]
    fn test_mod_inverse_non_coprime() {
        assert_eq!(mod_inverse(48, 18), None);
        assert_eq!(mod_inverse(18, 48), None);
        assert_eq!(mod_inverse(25, 25), None);
    }

    #[test]
    fn test_mod_inverse_zero() {
        assert_eq!(mod_inverse(0, 5), None);
        assert_eq!(mod_inverse(7, 0), None);
        assert_eq!(mod_inverse(0, 0), None);
    }

    #[test]
    fn test_mod_inverse_overflow() {
        assert_eq!(mod_inverse(u64::MAX, u64::MAX), None);
        assert_eq!(mod_inverse(u64::MAX, u64::MAX - 1), Some(1));
        assert_eq!(mod_inverse(u64::MAX, 2), Some(1));
        assert_eq!(mod_inverse(1, u64::MAX), Some(1));
    }
}
