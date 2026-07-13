fn main() {
    let (p, q, d) = (1073741789, 1073741827, 15000);
    let keys = rsa_keygen(p, q, d).unwrap();

    println!("p = {}, q = {}", p, q);
    println!("Public key: {:?}", keys.0);
    println!("Private key: {:?}", keys.1);

    for msg in 0..256 {
        let cipher = rsa_encrypt(msg, &keys.0).unwrap();
        let descrypted = rsa_decrypt(cipher, &keys.1).unwrap();

        if descrypted != msg {
            println!("Message: {}, Cipher: {}, Decrypted: {}", msg, cipher, descrypted);
        }
    }

    println!("All messages decrypted correctly.");

    let terms = decompose(keys.0.e, keys.0.n).unwrap();
    let convergents = convergents(&terms).unwrap();

    for (idx, (p, q)) in convergents.iter().enumerate() {
        if idx > 0 {
            println!("{}: {} / {}", idx, p, q);
        }
    }

    println!("Wiener attack: {:?}", wiener(&keys.0, &convergents));
}

fn wiener(key: &PubKey, convergents: &[(u64, u64)]) -> Option<(u64, u64)> {
    if convergents.is_empty() {
        return None;
    }

    for &(k, d) in convergents.iter() {
        if k == 0 || d == 0 {
            continue;
        }

        let nominator = key.e as u128 * d as u128 - 1;
        if nominator % k as u128 != 0 {
            continue;
        }

        let phi = (nominator / k as u128) as u64;
        if key.n + 1 < phi {
            continue;
        }

        let p_plus_q = key.n + 1 - phi;
        let p_plus_q_square = (p_plus_q as u128) * (p_plus_q as u128);

        if p_plus_q_square < 4 * key.n as u128 {
            continue;
        }

        let discriminator = (p_plus_q_square - 4 * key.n as u128) as u64;
        let sqrt = isqrt(discriminator);
        if sqrt * sqrt != discriminator {
            continue;
        }

        let t1 = (p_plus_q + sqrt) / 2;
        let t2 = (p_plus_q - sqrt) / 2;

        if t1 * t2 == key.n {
            return Some((t1, t2));
        }
    }

    return None;
}

fn isqrt(n: u64) -> u64 {
    let mut lo = 0;
    let mut hi = n;

    while lo < hi {
        let mid = lo + (hi - lo).saturating_add(1) / 2;

        match mid.checked_mul(mid) {
            Some(v) if v <= n => lo = mid,
            _ => hi = mid - 1,
        }
    }

    return lo;
}

fn convergents(terms: &[u64]) -> Option<Vec<(u64, u64)>> {
    if terms.is_empty() {
        return None;
    }

    let mut pq = vec![(1, 0), (terms[0], 1)];

    for idx in 1..terms.len() {
        let pk = terms[idx] * pq[idx].0 + pq[idx - 1].0;
        let qk = terms[idx] * pq[idx].1 + pq[idx - 1].1;

        pq.push((pk, qk));
    }

    return Some(pq);
}

fn decompose(mut a: u64, mut b: u64) -> Option<Vec<u64>> {
    let mut quotients = Vec::new();

    if b == 0 {
        return None;
    }

    while b != 0 {
        let temp = b;
        quotients.push(a / b);

        b = a % b;
        a = temp;
    }

    return Some(quotients);
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

#[derive(Debug, PartialEq)]
struct PubKey {
    n: u64,
    e: u64,
}

#[derive(Debug, PartialEq)]
struct PrivKey {
    n: u64,
    d: u64,
}

fn rsa_keygen(p: u64, q: u64, d: u64) -> Option<(PubKey, PrivKey)> {
    if p < 20 || q < 20 {
        return None;
    }

    // verify p and q, we relly on it
    for a in 2..20 {
        if let MillRabinResult::IsComposite = miller_rabin(a, p)? {
            return None;
        }

        if let MillRabinResult::IsComposite = miller_rabin(a, q)? {
            return None;
        }
    }

    let n = p * q;
    let phi = (p - 1) * (q - 1);
    let off = d;

    for d in off..phi {
        let e = match mod_inverse(d, phi) {
            Some(val) => val,
            None => continue,
        };

        if gcd(e, phi)?.0 == 1 {
            return Some((PubKey { n, e }, PrivKey { n, d }));
        }
    }

    return None;
}

fn rsa_encrypt(msg: u64, key: &PubKey) -> Option<u64> {
    return mod_exp(msg, key.e, key.n);
}

fn rsa_decrypt(cipher: u64, key: &PrivKey) -> Option<u64> {
    return mod_exp(cipher, key.d, key.n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        assert!(rsa_keygen(23, 29, 3).is_some());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let (pub_key, priv_key) = rsa_keygen(23, 29, 3).unwrap();

        let cipher = rsa_encrypt(19, &pub_key).unwrap();
        assert_eq!(rsa_decrypt(cipher, &priv_key), Some(19));
    }

        #[test]
    fn test_decompose() {
        assert_eq!(decompose(17, 7), Some(vec![2, 2, 3]));
        assert_eq!(decompose(13, 8), Some(vec![1, 1, 1, 1, 2]));
    }

    #[test]
    fn test_decompose_zero() {
        assert_eq!(decompose(0, 5), Some(vec![0]));
        assert_eq!(decompose(7, 0), None);
    }

    #[test]
    fn test_convergents_17_by_7() {
        let terms = decompose(17, 7).unwrap();
        let convergents = convergents(&terms).unwrap();

        assert_eq!(convergents[0], (1, 0));
        assert_eq!(convergents[1], (2, 1));
        assert_eq!(convergents[2], (5, 2));
        assert_eq!(convergents[3], (17, 7));
    }

    #[test]
    fn test_convergents_13_by_8() {
        let terms = decompose(13, 8).unwrap();
        let convergents = convergents(&terms).unwrap();

        assert_eq!(convergents[0], (1, 0));
        assert_eq!(convergents[1], (1, 1));
        assert_eq!(convergents[2], (2, 1));
        assert_eq!(convergents[3], (3, 2));
        assert_eq!(convergents[4], (5, 3));
        assert_eq!(convergents[5], (13, 8));
    }

        #[test]
    fn test_isqrt_low() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(2), 1);
        assert_eq!(isqrt(3), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(5), 2);
        assert_eq!(isqrt(6), 2);
        assert_eq!(isqrt(7), 2);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(10), 3);
        assert_eq!(isqrt(11), 3);
        assert_eq!(isqrt(12), 3);
        assert_eq!(isqrt(13), 3);
        assert_eq!(isqrt(14), 3);
        assert_eq!(isqrt(15), 3);
        assert_eq!(isqrt(16), 4);
        assert_eq!(isqrt(17), 4);
        assert_eq!(isqrt(26), 5);
    }

    #[test]
    fn test_isqrt_mid() {
        assert_eq!(isqrt(999_999), 999);
        assert_eq!(isqrt(1_000_000), 1000);
        assert_eq!(isqrt(1_000_001), 1000);
        assert_eq!(isqrt(1_000_002), 1000);

        assert_eq!(isqrt(1_048_575), 1023);
        assert_eq!(isqrt(1_048_576), 1024);
        assert_eq!(isqrt(1_048_577), 1024);
        assert_eq!(isqrt(1_048_578), 1024);
        assert_eq!(isqrt(1_048_579), 1024);
    }

    #[test]
    fn test_isqrt_high() {
        assert_eq!(isqrt(u64::MAX - 2), 4_294_967_295);
        assert_eq!(isqrt(u64::MAX - 1), 4_294_967_295);
        assert_eq!(isqrt(u64::MAX), 4_294_967_295);
    }

    #[test]
    fn test_wiener_39() {
        let (p, q, d) = (1073741789, 1073741827, 39);
        let keys = rsa_keygen(p, q, d).unwrap();

        let terms = decompose(keys.0.e, keys.0.n).unwrap();
        let convergents = convergents(&terms).unwrap();

        let pq = wiener(&keys.0, &convergents);
        assert!(pq == Some((p, q)) || pq == Some((q, p)));
    }

    #[test]
    fn test_wiener_21000() {
        let (p, q, d) = (1073741789, 1073741827, 21000);
        let keys = rsa_keygen(p, q, d).unwrap();

        let terms = decompose(keys.0.e, keys.0.n).unwrap();
        let convergents = convergents(&terms).unwrap();

        let pq = wiener(&keys.0, &convergents);
        assert!(pq == Some((p, q)) || pq == Some((q, p)));
    }

    #[test]
    fn test_wiener_200000() {
        let (p, q, d) = (1073741789, 1073741827, 200000);
        let keys = rsa_keygen(p, q, d).unwrap();

        let terms = decompose(keys.0.e, keys.0.n).unwrap();
        let convergents = convergents(&terms).unwrap();

        assert_eq!(wiener(&keys.0, &convergents), None);
    }
}
