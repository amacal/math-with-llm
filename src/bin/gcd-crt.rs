fn main() {
    println!("Chinese Remainder Theorem solution of 3 mod 17 and 4 mod 19 is: {:?}", solve_crt((3, 17), (4, 19)));
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

fn solve_crt(mut x: (u64, u64), mut y: (u64, u64)) -> Option<u64> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_crt() {
        assert_eq!(solve_crt((3, 5), (2, 3)), Some(8));
        assert_eq!(solve_crt((5, 7), (1, 3)), Some(19));
    }

    #[test]
    fn test_solve_crt_non_coprime() {
        assert_eq!(solve_crt((3, 18), (7, 48)), None);
        assert_eq!(solve_crt((1, 25), (3, 25)), None);
    }

    #[test]
    fn test_solve_crt_zero() {
        assert_eq!(solve_crt((0, 5), (7, 0)), None);
        assert_eq!(solve_crt((7, 0), (3, 5)), None);
    }

    #[test]
    fn test_solve_crt_overflow() {
        assert_eq!(solve_crt((3, u64::MAX), (1, u64::MAX)), None);
    }
}
