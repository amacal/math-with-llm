fn main() {
    const P: u64 = 18_446_744_073_709_551_557;
    let mut counter = 0;

    for p in [P] {
        for n in P/2..P {
            match mod_sqrt(n, p) {
                Some(value) => {
                    println!("mod_sqrt({}, {}) = {}", n, p, value);
                    counter += 1;
                },
                _ => {},
            }

            if counter == 10 {
                break;
            }
        }
    }
}

fn mod_sqrt(a: u64, p: u64) -> Option<u64> {
    if a == 0 || a >= p {
        return None;
    }

    if legendre(a, p) != Some(1) {
        return None;
    }

    let mut q = p-1;
    let mut s = 0;
    let mut z = 2;

    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    while legendre(z, p) != Some(p - 1) {
        z += 1;
    }

    let mut r = mod_exp(a, (q+1)/2, p)?;
    let mut t = mod_exp(a, q, p)?;
    let mut c = mod_exp(z, q, p)?;
    let mut m = s;

    fn multiply(x: u64, y: u64, p: u64) -> u64 {
        return ((x as u128 * y as u128) % p as u128) as u64;
    }

    while t != 1 {
        let mut i = 1;
        let mut t2i = multiply(t, t, p);

        while t2i != 1 {
            t2i = multiply(t2i, t2i, p);
            i += 1;
        }

        let h = mod_exp(c, 1 << (m - i - 1), p)?;

        r = multiply(r, h, p);
        t = multiply(t, multiply(h, h, p), p);
        c = multiply(h, h, p);
        m = i;
    }

    return Some(r);
}

fn legendre(n: u64, p: u64) -> Option<u64> {
    if p == 0 {
        return None;
    }

    // the function assumes that p is prime,
    // so we can use Euler's criterion to compute the Legendre symbol
    return mod_exp(n, (p - 1) / 2, p);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_roots_in_ring(n: u64) {
        let mut roots = 0;

        for i in 1..n {
            match mod_sqrt(i, n) {
                Some(sqrt) => {
                    assert_eq!((sqrt * sqrt) % n, i);
                    roots += 1;
                },
                None => {
                    assert_ne!(legendre(i, n), Some(1));
                }
            }
        }

        assert_eq!(roots, (n - 1) / 2);
    }

    #[test]
    fn can_square_ring() {
        for i in [3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41] {
            assert_roots_in_ring(i);
        }
    }

    #[test]
    fn can_square_edge() {
        assert_eq!(mod_sqrt(0, 7), None);
        assert_eq!(mod_sqrt(1, 0), None);
        assert_eq!(mod_sqrt(13, 7), None);
    }
}
