fn main() {
    for i in 1..=100u64 {
        match factorize(u64::MAX - i) {
            Some(factors) => println!("{}: {:?}", i, factors),
            None => panic!("Failed to factorize {}", u64::MAX - i),
        }
    }
}

fn factorize(mut n: u64) -> Option<Vec<u64>> {
    let mut divisor = 2;
    let mut factors: Vec<u64> = Vec::new();

    if n == 0 {
        return None;
    }

    while n > 1 {
        if n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        } else {
            divisor += 1;
        }

        match divisor.checked_mul(divisor) {
            Some(val) if val > n => break,
            None => break,
            _ => {}
        }
    }

    if n > 1 {
        factors.push(n);
    }

    return Some(factors);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorize_prime() {
        assert_eq!(factorize(1), Some(vec![]));
        assert_eq!(factorize(2), Some(vec![2]));
        assert_eq!(factorize(3), Some(vec![3]));
        assert_eq!(factorize(5), Some(vec![5]));
        assert_eq!(factorize(7), Some(vec![7]));
    }

    #[test]
    fn test_factorize_composite() {
        assert_eq!(factorize(1), Some(vec![]));
        assert_eq!(factorize(2), Some(vec![2]));
        assert_eq!(factorize(6), Some(vec![2, 3]));
        assert_eq!(factorize(12), Some(vec![2, 2, 3]));
    }

    #[test]
    fn test_factorize_zero() {
        assert_eq!(factorize(0), None);
    }

    #[test]
    fn test_factorize_large() {
        assert_eq!(factorize(u64::MAX), Some(vec![3, 5, 17, 257, 641, 65537, 6700417]));
        assert_eq!(factorize(u64::MAX-1), Some(vec![2, 7, 7, 73, 127, 337, 92737, 649657]));
    }
}
