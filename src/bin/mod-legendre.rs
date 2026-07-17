fn main() {
    for p in [3, 5, 7] {
        for n in 0..p {
            match legendre(n, p) {
                Some(value) => println!("legendre({}, {}) = {}", n, p, value),
                _ => {},
            }
        }
    }
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

    #[test]
    fn test_legendre() {
        assert_eq!(legendre(0, 7), Some(0));
        assert_eq!(legendre(1, 7), Some(1));
        assert_eq!(legendre(2, 7), Some(1));
        assert_eq!(legendre(3, 7), Some(6));
        assert_eq!(legendre(4, 7), Some(1));
        assert_eq!(legendre(5, 7), Some(6));
        assert_eq!(legendre(6, 7), Some(6));
    }

    #[test]
    fn test_legendre_invalid() {
        assert_eq!(legendre(2, 0), None);
    }
}
