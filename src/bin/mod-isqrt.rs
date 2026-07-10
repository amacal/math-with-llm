fn main() {
    println!("isqrt(3,423,426,346) = {}", isqrt(3423426346));
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
