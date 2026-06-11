fn main() {
    println!("GCD of 48 and 18 is: {:?}", gcd(48, 18));
}

fn gcd(mut a: u64, mut b: u64) -> Option<u64> {
    // edge case if values are both zero, GCD is undefined
    if a == 0 && b == 0 {
        return None;
    }

    // track the number of common factors of 2
    let mut k: u64 = 0;

    // repeatedly try to reduce both numbers until one of them becomes zero
    while b > 0 && a > 0 {
        (a, b, k) = match (a & 1, b & 1, a > b) {
            (0, 0, _) => (a >> 1, b >> 1, k + 1),
            (0, _, _) => (a >> 1, b, k),
            (_, 0, _) => (a, b >> 1, k),
            (_, _, true) => (a - b, b, k),
            (_, _, false) => (a, b - a, k),
        }
    }

    // either a or b is zero, so the GCD of the remaining number multiplied by 2^k
    Some((a + b) << k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), Some(6));
        assert_eq!(gcd(18, 48), Some(6));

        assert_eq!(gcd(15, 22), Some(1));
        assert_eq!(gcd(22, 15), Some(1));

        assert_eq!(gcd(17, 19), Some(1));
        assert_eq!(gcd(19, 17), Some(1));

        assert_eq!(gcd(25, 25), Some(25));
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(0, 5), Some(5));
        assert_eq!(gcd(7, 0), Some(7));
        assert_eq!(gcd(0, 0), None);
    }
}
