fn main() {
    println!("GCD of 48 and 18 is: {:?}", gcd(48, 18));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), Some((6, (-1, 3))));
        assert_eq!(gcd(18, 48), Some((6, (3, -1))));

        assert_eq!(gcd(15, 22), Some((1, (3, -2))));
        assert_eq!(gcd(22, 15), Some((1, (-2, 3))));

        assert_eq!(gcd(17, 19), Some((1, (9, -8))));
        assert_eq!(gcd(19, 17), Some((1, (-8, 9))));

        assert_eq!(gcd(25, 25), Some((25, (0, 1))));
    }

    #[test]
    fn test_gcd_zero() {
        assert_eq!(gcd(0, 5), Some((5, (0, 1))));
        assert_eq!(gcd(7, 0), Some((7, (1, 0))));
        assert_eq!(gcd(0, 0), None);
    }

    #[test]
    fn test_gcd_overflow() {
        assert_eq!(gcd(u64::MAX, u64::MAX), Some((u64::MAX, (0, 1))));
        assert_eq!(gcd(u64::MAX, 1), Some((1, (0, 1))));
        assert_eq!(gcd(1, u64::MAX), Some((1, (1, 0))));
    }
}
