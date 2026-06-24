fn main() {
    factorize(u16::MIN as u64..u16::MAX as u64, 8);
    factorize(u32::MIN as u64..u32::MAX as u64, 8);
    factorize(u64::MIN..u64::MAX, 8);
}

fn factorize(range: std::ops::Range<u64>, mut counter: u32) {
    let mut numbers: Option<(u64, u64, u64)>;

    for i in range.rev() {
        (counter, numbers) = match (counter, pollard_rho(i as u64)) {
            (0, _) => return,
            (cnt, Some(PollardRhoResult::Factor { d, c, r, .. })) if d as f64 > (i as f64).sqrt() => (cnt - 1, Some((d, c, r))),
            (cnt, _) => (cnt, None),
        };

        if let Some((d, c, r)) = numbers {
            println!("{} has factor {} with c = {} and r = {}", i, d, c, r);
        }
    }
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

    const LIMIT: u64 = 1048576;
    const ATTEMPTS: u64 = 10;
    const PRIMES: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    for a in 2..std::cmp::min(n, ATTEMPTS) {
        match (a, miller_rabin(a, n)) {
            (_, Some(MillRabinResult::IsComposite)) => break,
            (v, _) if v == ATTEMPTS - 1 || v == n => return Some(PollardRhoResult::Prime { n }),
            _ => (),
        }
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

    fn extract(result: Option<PollardRhoResult>) -> Option<u64> {
        match result? {
            PollardRhoResult::Factor { d, .. } => Some(d),
            PollardRhoResult::Prime { n } => Some(n),
        }
    }

    #[test]
    fn test_pollard_rho_composite() {
        assert_eq!(extract(pollard_rho(15)).map(|x| x == 3 || x == 5), Some(true));
        assert_eq!(extract(pollard_rho(21)).map(|x| x == 3 || x == 7), Some(true));
        assert_eq!(extract(pollard_rho(35)).map(|x| x == 5 || x == 7), Some(true));
        assert_eq!(extract(pollard_rho(77)).map(|x| x == 7 || x == 11), Some(true));

        assert_eq!(extract(pollard_rho(4)), Some(2));
        assert_eq!(extract(pollard_rho(49)), Some(7));
        assert_eq!(extract(pollard_rho(121)), Some(11));
    }

    #[test]
    fn test_pollard_rho_prime() {
        assert_eq!(extract(pollard_rho(2)).unwrap_or(2), 2);
        assert_eq!(extract(pollard_rho(3)).unwrap_or(3), 3);
        assert_eq!(extract(pollard_rho(5)).unwrap_or(5), 5);
        assert_eq!(extract(pollard_rho(7)).unwrap_or(7), 7);
        assert_eq!(extract(pollard_rho(11)).unwrap_or(11), 11);
        assert_eq!(extract(pollard_rho(13)).unwrap_or(13), 13);
        assert_eq!(extract(pollard_rho(17)).unwrap_or(17), 17);
        assert_eq!(extract(pollard_rho(19)).unwrap_or(19), 19);
    }

    #[test]
    fn test_pollard_rho_zero() {
        assert_eq!(pollard_rho(0), None);
    }

    #[test]
    fn test_pollard_rho_overflow() {
        assert!(pollard_rho(u64::MAX).is_some());
        assert!(pollard_rho(u64::MAX - 1).is_some());
    }
}
