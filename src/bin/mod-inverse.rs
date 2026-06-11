fn main() {
    println!("Modular inverse of 17 and 19 is: {:?}", mod_inverse(17, 19));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_inverse() {
        assert_eq!(mod_inverse(15, 22), Some(3));
        assert_eq!(mod_inverse(22, 15), Some(13));

        assert_eq!(mod_inverse(17, 19), Some(9));
        assert_eq!(mod_inverse(19, 17), Some(9));
    }

    #[test]
    fn test_mod_inverse_non_coprime() {
        assert_eq!(mod_inverse(48, 18), None);
        assert_eq!(mod_inverse(18, 48), None);
        assert_eq!(mod_inverse(25, 25), None);
    }

    #[test]
    fn test_mod_inverse_zero() {
        assert_eq!(mod_inverse(0, 5), None);
        assert_eq!(mod_inverse(7, 0), None);
        assert_eq!(mod_inverse(0, 0), None);
    }

    #[test]
    fn test_mod_inverse_overflow() {
        assert_eq!(mod_inverse(u64::MAX, u64::MAX), None);
        assert_eq!(mod_inverse(u64::MAX, u64::MAX - 1), Some(1));
        assert_eq!(mod_inverse(u64::MAX, 1), Some(0));
        assert_eq!(mod_inverse(1, u64::MAX), Some(1));
    }
}
