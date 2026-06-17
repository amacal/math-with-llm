fn main() {
    let keys = rsa_keygen(257, 263).unwrap();

    println!("Public key: {:?}", keys.0);
    println!("Private key: {:?}", keys.1);

    for msg in 0..256 {
        let cipher = rsa_encrypt(msg, &keys.0).unwrap();
        let descrypted = rsa_decrypt(cipher, &keys.1).unwrap();

        println!("Message: {}, Cipher: {}, Decrypted: {}", msg, cipher, descrypted);
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

fn rsa_keygen(p: u64, q: u64) -> Option<(PubKey, PrivKey)> {
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

    for e in 3..phi {
        if gcd(e, phi)?.0 == 1 {
            let d = mod_inverse(e, phi)?;
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
        assert!(rsa_keygen(23, 29).is_some());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let (pub_key, priv_key) = rsa_keygen(23, 29).unwrap();

        let cipher = rsa_encrypt(19, &pub_key).unwrap();
        assert_eq!(rsa_decrypt(cipher, &priv_key), Some(19));
    }
}
