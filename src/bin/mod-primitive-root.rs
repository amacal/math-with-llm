use std::collections::HashSet;

fn main() {
    let mut counter = 100;
    let mut number = 0xffffffffffffffff;

    while counter > 0 {
        number -= 1;

        match primitive_root(number) {
            Some(root) => println!("Primitive root of {}: {}", number, root),
            None => continue,
        }

        counter -= 1;
    }
}

fn primitive_root(p: u64) -> Option<u64> {
    if p <= 2 {
        return None;
    }

    if let Some(false) = miller_test(p) {
        return None;
    }

    let factors = factorize(p - 1);

    for g in 2..p {
        let mut found = true;

        for &factor in factors.iter() {
            let exp = (p - 1) / factor;
            let res = mod_exp(g, exp, p)?;

            if res == 1 {
                found = false;
                break;
            }
        }

        if found {
            return Some(g);
        }
    }

    return None;
}

fn factorize(n: u64) -> Vec<u64> {
    let mut factors = HashSet::new();
    let mut candidates = vec![n; 1];

    while let Some(n) = candidates.pop() {
        match miller_test(n) {
            Some(true) => {
                factors.insert(n);
                continue;
            }
            None => break,
            _ => (),
        }

        match pollard_rho(n) {
            Some(PollardRhoResult::Factor { d, .. }) => {
                candidates.push(d);
                candidates.push(n / d);
            }
            Some(PollardRhoResult::Prime { .. }) => {
                factors.insert(n);
            }
            None => {}
        }
    }

    let mut result: Vec<u64> = factors.iter().cloned().collect();

    result.sort();
    return result;
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

fn miller_test(n: u64) -> Option<bool> {
    const ATTEMPTS: u64 = 10;

    if n == 0 || n == 1 {
        return None;
    }

    for a in 2..std::cmp::min(n, ATTEMPTS) {
        match (a, miller_rabin(a, n)) {
            (_, Some(MillRabinResult::IsComposite)) => return Some(false),
            (v, _) if v == ATTEMPTS - 1 || v == n => return Some(true),
            _ => (),
        }
    }

    return Some(true);
}

#[derive(Debug, PartialEq)]
enum PollardRhoResult {
    Factor { n: u64, d: u64, c: u64, r: u64 },
    Prime { n: u64 },
}

fn pollard_rho(n: u64) -> Option<PollardRhoResult> {
    if n == 0 || n == 1 {
        return None;
    }

    if n % 2 == 0 {
        return Some(PollardRhoResult::Factor { n, d: 2, c: 0, r: 0 });
    }

    if let Some(true) = miller_test(n) {
        return Some(PollardRhoResult::Prime { n });
    }

    fn fx(x: u128, c: u128, n: u128) -> u128 {
        return (x * x % n + c) % n;
    }

    fn abs(a: u128, b: u128) -> u128 {
        return if a > b { a - b } else { b - a };
    }

    fn attempt(c: u128, n: u64, mut limit: u64) -> (Option<u64>, u64) {
        let mut hase: u128 = 0;
        let mut tortoise: u128 = 1;

        while hase != tortoise {
            let diff = abs(hase, tortoise);
            let diff = (diff % (n as u128)) as u64;

            match gcd(diff, n) {
                Some(1) => (),
                Some(d) => return (Some(d), limit),
                None => return (None, limit),
            }

            hase = fx(hase, c, n as u128);
            tortoise = fx(tortoise, c, n as u128);
            tortoise = fx(tortoise, c, n as u128);

            limit = match limit {
                0 => return (None, limit),
                _ => limit - 1,
            };
        }

        return (None, limit);
    }

    const LIMIT: u64 = 1048576;
    const PRIMES: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    let mut rounds = 0;
    for &c in PRIMES.iter() {
        rounds += match (rounds, attempt(c as u128, n, LIMIT)) {
            (rounds, (Some(factor), limit)) if factor < n => return Some(PollardRhoResult::Factor { n, d: factor, c, r: rounds + LIMIT - limit }),
            (rounds, (_, limit)) => rounds + LIMIT - limit,
        };
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize_small() {
        assert_eq!(factorize(1), vec![]);
        assert_eq!(factorize(2), vec![2]);
        assert_eq!(factorize(3), vec![3]);
        assert_eq!(factorize(4), vec![2]);
        assert_eq!(factorize(5), vec![5]);
        assert_eq!(factorize(6), vec![2, 3]);
        assert_eq!(factorize(7), vec![7]);
        assert_eq!(factorize(8), vec![2]);
        assert_eq!(factorize(9), vec![3]);
        assert_eq!(factorize(10), vec![2, 5]);
    }

    #[test]
    fn test_factorize_large() {
        assert_eq!(factorize(0xffffffffffffffff), vec![3, 5, 17, 257, 641, 65537, 6700417]);
        assert_eq!(factorize(0x123456789abcdef), vec![3, 5, 59, 1051, 88143687977]);
    }

    #[test]
    fn test_primitive_root_small() {
        assert_eq!(primitive_root(7), Some(3));
        assert_eq!(primitive_root(11), Some(2));
        assert_eq!(primitive_root(13), Some(2));
        assert_eq!(primitive_root(17), Some(3));
        assert_eq!(primitive_root(19), Some(2));
    }

    #[test]
    fn test_primitive_root_large() {
        assert_eq!(primitive_root(0xfffffffffffffcc7), Some(11));
        assert_eq!(primitive_root(0xfffffffffffffc4f), Some(6));
    }

    #[test]
    fn test_primitive_root_guard() {
        assert_eq!(primitive_root(0), None);
        assert_eq!(primitive_root(1), None);
        assert_eq!(primitive_root(2), None);
    }

}
