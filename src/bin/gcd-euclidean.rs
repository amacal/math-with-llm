fn main() {
    println!("GCD of 48 and 18 is: {:?}", gcd(48, 18));
}

fn gcd(mut a: u64, mut b: u64) -> Option<u64> {
    if a == 0 && b == 0 {
        return None;
    }

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    Some(a)
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
