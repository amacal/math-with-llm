fn main() {
    for t in 1..10 {
        let mut false_positive = 0;

        for i in 1024..1048576 {
            let mut maybe = 0;
            let mut composite = 0;

            for j in 1..t {
                (maybe, composite) = match miller_rabin(i/2 + j, i) {
                    Some(MillRabinResult::MaybePrime) => (maybe + 1, composite),
                    Some(MillRabinResult::IsComposite) => (maybe, composite + 1),
                    None => panic!("Unexpected None result"),
                }
            }

            if composite == 0 && totient(i) != i - 1 {
                false_positive += 1;
            }
        }

        println!("Checks {}: False Positives: {}", t, false_positive);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin_561() {
        let mut maybe_prime_count: u64 = 0;

        for i in 2..560 {
            match miller_rabin(i, 561) {
                Some(MillRabinResult::MaybePrime) => maybe_prime_count += 1,
                Some(MillRabinResult::IsComposite) => (),
                None => panic!("Unexpected None result"),
            }
        }

        assert_eq!(maybe_prime_count > 0, true);
        assert_eq!(maybe_prime_count < 560, true);
    }

    #[test]
    fn test_miller_rabin_999() {
        for i in 2..998 {
            assert_eq!(miller_rabin(i, 999), Some(MillRabinResult::IsComposite));
        }
    }

    #[test]
    fn test_miller_rabin_7() {
        for i in 2..6 {
            assert_eq!(miller_rabin(i, 7), Some(MillRabinResult::MaybePrime));
        }
    }

    #[test]
    fn test_miller_rabin_71() {
        for i in 2..70 {
            assert_eq!(miller_rabin(i, 71), Some(MillRabinResult::MaybePrime));
        }
    }

    #[test]
    fn test_miller_rabin_zero() {
        assert_eq!(miller_rabin(0, 5), None);
        assert_eq!(miller_rabin(7, 0), None);
        assert_eq!(miller_rabin(0, 0), None);
    }

    #[test]
    fn test_miller_rabin_overflow() {
        assert_eq!(miller_rabin(u64::MAX, u64::MAX), None);
        assert_eq!(miller_rabin(u64::MAX-1, u64::MAX), Some(MillRabinResult::MaybePrime));
        assert_eq!(miller_rabin(1, u64::MAX), Some(MillRabinResult::MaybePrime));
    }
}
