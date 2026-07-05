use std::cmp::min;
use std::collections::HashSet;
use std::vec;

fn main() {
    let a = [1, 2, 3, 0, 0, 0, 0, 0];
    let b = [2, 3, 1, 0, 0, 0, 0, 0];

    println!("Convolution of {:?} and {:?} is {:?}", a, b, convolution_naive(&a, &b));
    println!("Convolution of {:?} and {:?} is {:?}", a, b, convolution_ntt(&a, &b));
}

fn convolution_naive<const T: usize>(a: &[u64; T], b: &[u64; T]) -> Option<Vec<u64>> {
    if T == 0 {
        return None;
    }

    let (n, m) = (a.len(), b.len());
    let mut result = vec![0; m + n - 1];

    for k in 0..result.len() {
        let start = (1 + k).saturating_sub(m) as usize;
        let end = min(k, n.saturating_sub(1)) as usize;

        for i in start..=end {
            result[k] += a[i] * b[k - i];
        }
    }

    return Some(result);
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

fn gcd(mut a: u128, mut b: u128) -> Option<(u128, (i128, i128))> {
    // edge case: gcd(0, 0) is undefined
    if a == 0 && b == 0 {
        return None;
    }

    // seed values
    let mut q;
    let (mut xa, mut ya): (i128, i128) = (1, 0);
    let (mut xb, mut yb): (i128, i128) = (0, 1);

    // stop when a is already the gcd
    while b != 0 {
        let (ta, tx, ty) = (b, xb, yb);
        (b, q) = (a % b, a / b);

        // if the cast is unsuccessful, it means the q doesn't fit i128, implying that b is 0
        // and we have found the gcd, skipping the update of the linear coefficients
        if let Ok(q) = i128::try_from(q) {
            (xb, yb) = (xa - q * xb, ya - q * yb);
        }

        (a, xa, ya) = (ta, tx, ty);
    }

    // determined gcd and its linear coefficients
    Some((a, (xa, ya)))
}

fn mod_inverse(a: u128, m: u128) -> Option<u128> {
    return match gcd(a, m) {
        Some((1, (x, _))) => match (u128::try_from(x), u128::try_from(-x)) {
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

            match gcd(diff as u128, n as u128) {
                Some((1, _)) => (),
                Some((d, _)) => return (Some(d as u64), limit),
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

fn solve_crt(mut x: (u128, u128), mut y: (u128, u128)) -> Option<u128> {
    // catch edge cases
    if x.1 == 0 || y.1 == 0 {
        return None;
    }

    // normalize x if needed
    if x.0 >= x.1 {
        x.0 = x.0 % x.1;
    }

    // normalize y if needed
    if y.0 >= y.1 {
        y.0 = y.0 % y.1;
    }

    // if the moduli overflow we cannot continue
    let m = x.1.checked_mul(y.1)?;

    // find the modular inverse or safely fail
    let inv = mod_inverse(x.1, y.1)?;

    // safely apply the CRT formula to avoid overflow
    let k = inv.checked_mul(match y.0 >= x.0 {
        true => y.0 - x.0,
        false => (y.1 - (x.0 - y.0) % y.1) % y.1,
    })?;

    // cumpute the solution or safely fail
    return Some((k.checked_mul(x.1)? % m).checked_add(x.0)? % m);
}

fn convolution_ntt(a: &[u64; 8], b: &[u64; 8]) -> Option<[u128; 16]> {
    let primes: [u64; 3] = [998244353, 985661441, 469762049];
    let mut results: [[u64; 16]; 3] = [[0; 16]; 3];
    let mut solution: [u128; 16] = [0; 16];

    for (idx, &prime) in primes.iter().enumerate() {
        let p_forward: u64 = prime;
        let p_inverse: u64 = mod_inverse(16, p_forward as u128)? as u64;

        let p_root_forward = primitive_root(p_forward)?;
        let p_root_inverse = mod_inverse(p_root_forward as u128, p_forward as u128)? as u64;

        let mut p_roots_forward = [0u64; 16];
        let mut p_roots_inverse = [0u64; 16];

        let mut p_root_acc = 1;
        let p_root_step = mod_exp(p_root_forward, (p_forward - 1) / 16, p_forward)?;

        for i in 0..16 {
            p_roots_forward[i] = p_root_acc;
            p_root_acc = p_root_acc * p_root_step % p_forward;
        }

        let mut p_root_acc = 1;
        let p_root_step = mod_exp(p_root_inverse, (p_forward - 1) / 16, p_forward)?;

        for i in 0..16 {
            p_roots_inverse[i] = p_root_acc;
            p_root_acc = p_root_acc * p_root_step % p_forward;
        }

        let mut a2 = [0u64; 16];
        let mut b2 = [0u64; 16];

        for i in 0..8 {
            a2[i] = a[i];
            b2[i] = b[i];
        }

        ntt(&mut a2, 0, 1, &p_roots_forward, p_forward);
        ntt(&mut b2, 0, 1, &p_roots_forward, p_forward);

        for i in 0..16 {
            results[idx][i] = a2[i] * b2[i] % p_forward;
        }

        ntt_inverse(&mut results[idx], 0, 1, &p_roots_inverse, p_forward, p_inverse);

        if idx > 0 {
            let mut prime_in_solution: u128 = 1;

            for i in 0..idx {
                prime_in_solution = prime_in_solution * primes[i] as u128;
            }

            for i in 0..16 {
                solution[i] = solve_crt((solution[i], prime_in_solution), (results[idx][i] as u128, primes[idx] as u128))?;
            }
        } else {
            for i in 0..16 {
                solution[i] = results[0][i] as u128;
            }
        }
    }

    return Some(solution);
}

fn ntt_inverse<const T: usize>(coeffs: &mut [u64; T], offset: usize, stride: usize, roots: &[u64; T], p: u64, inv: u64) {
    ntt(coeffs, offset, stride, roots, p);

    for i in 0..coeffs.len() {
        coeffs[i] = coeffs[i] * inv % p;
    }
}

fn ntt<const T: usize>(coeffs: &mut [u64; T], offset: usize, stride: usize, roots: &[u64; T], p: u64) {
    let mut root = 0;
    let size: usize = (coeffs.len() / stride) >> 1;
    let mut scratch: [u64; T] = [0; T];

    if size == 0 {
        return;
    }

    ntt(coeffs, offset, stride << 1, roots, p);
    ntt(coeffs, offset + stride, stride << 1, roots, p);

    for i in 0..size {
        let e = coeffs[offset + i * (stride << 1)];
        let o = coeffs[offset + i * (stride << 1) + stride];
        let w = roots[root] * o % p;

        scratch[offset + i * stride] = (e + w) % p;
        scratch[offset + i * stride + size * stride] = (e + p - w) % p;

        root = (root + stride) % roots.len();
    }

    for i in 0..size {
        coeffs[offset + i * stride] = scratch[offset + i * stride];
        coeffs[offset + i * stride + size * stride] = scratch[offset + i * stride + size * stride];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolution_naive_small() {
        let a = [1, 2, 3, 0, 0, 0, 0, 0];
        let b = [2, 3, 1, 0, 0, 0, 0, 0];

        assert_eq!(convolution_naive(&a, &b), Some(vec![2, 7, 13, 11, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_convolution_ntt_small() {
        let a = [1, 2, 3, 0, 0, 0, 0, 0];
        let b = [2, 3, 1, 0, 0, 0, 0, 0];

        assert_eq!(convolution_ntt(&a, &b), Some([2, 7, 13, 11, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_convolution_naive_medium() {
        let a = [123, 456, 789, 234, 567, 890, 123, 456];
        let b = [231, 564, 897, 342, 675, 908, 231, 564];

        assert_eq!(
            convolution_naive(&a, &b),
            Some(vec![28413, 174708, 549774, 950148, 1209663, 1424598, 2094036, 2216022, 1706535, 2065734, 1310050, 944862, 944421, 174708, 257184])
        );
    }

    #[test]
    fn test_convolution_ntt_medium() {
        let a = [123, 456, 789, 234, 567, 890, 123, 456];
        let b = [231, 564, 897, 342, 675, 908, 231, 564];

        assert_eq!(
            convolution_ntt(&a, &b),
            Some([28413, 174708, 549774, 950148, 1209663, 1424598, 2094036, 2216022, 1706535, 2065734, 1310050, 944862, 944421, 174708, 257184, 0])
        );
    }

    #[test]
    fn test_convolution_naive_large() {
        let a = [123456789, 987654321, 111111111, 222222222, 333333333, 444444444, 555555555, 666666666];
        let b = [987654321, 123456789, 222222222, 111111111, 444444444, 333333333, 555555555, 666666666];

        assert_eq!(
            convolution_naive(&a, &b),
            Some(vec![
                121932631112635269,
                990702636540161562,
                259106841975461058,
                466392317533607682,
                545953359454046640,
                1021947871978052127,
                1149519887850480111,
                1629629626370370372,
                1197530861802469137,
                716049381283950618,
                802469134197530865,
                950617282049382717,
                827160492172839507,
                740740739259259260,
                444444443555555556
            ])
        );
    }

    #[test]
    fn test_convolution_ntts_large() {
        let a = [123456789, 987654321, 111111111, 222222222, 333333333, 444444444, 555555555, 666666666];
        let b = [987654321, 123456789, 222222222, 111111111, 444444444, 333333333, 555555555, 666666666];

        assert_eq!(
            convolution_ntt(&a, &b),
            Some([
                121932631112635269,
                990702636540161562,
                259106841975461058,
                466392317533607682,
                545953359454046640,
                1021947871978052127,
                1149519887850480111,
                1629629626370370372,
                1197530861802469137,
                716049381283950618,
                802469134197530865,
                950617282049382717,
                827160492172839507,
                740740739259259260,
                444444443555555556,
                0
            ])
        );
    }

    #[test]
    fn test_ntt_p5_forward() {
        let mut coeffs = [1, 2, 3, 1];
        let roots = [1, 2, 4, 3];

        ntt(&mut coeffs, 0, 1, &roots, 5);
        assert_eq!(coeffs, [2, 0, 1, 1]);
    }

    #[test]
    fn test_ntt_p5_inverse() {
        let mut coeffs = [2, 0, 1, 1];
        let roots = [1, 3, 4, 2];

        ntt_inverse(&mut coeffs, 0, 1, &roots, 5, 4);
        assert_eq!(coeffs, [1, 2, 3, 1]);
    }

    #[test]
    fn test_ntt_p17_forward() {
        let mut coeffs = [1, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let roots = [1, 3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6];

        ntt(&mut coeffs, 0, 1, &roots, 17);
        assert_eq!(coeffs, [7, 10, 5, 12, 11, 7, 1, 0, 1, 12, 7, 12, 2, 9, 8, 14]);
    }

    #[test]
    fn test_ntt_p17_inverse() {
        let mut coeffs = [7, 10, 5, 12, 11, 7, 1, 0, 1, 12, 7, 12, 2, 9, 8, 14];
        let roots = [1, 6, 2, 12, 4, 14, 8, 9, 16, 11, 15, 5, 13, 10, 3, 7];

        ntt_inverse(&mut coeffs, 0, 1, &roots, 17, 16);
        assert_eq!(coeffs, [1, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
