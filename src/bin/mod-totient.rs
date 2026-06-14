fn main() {
    println!("Euler's totient function of 184467440741 is: {:?}", totient(184467440741));
}

fn totient(mut n: u64) -> u64 {
    let mut result: u64 = 1;
    let mut p = 2;

    while p * p <= n {
        let mut k: u64 = 0;
        let mut t: u64 = 1;

        while n % p == 0 {
            n /= p;
            k += 1;
        }

        while k > 0 {
            t *= p;
            k -= 1;
        }

        if t > 1 {
            result *= t - t / p;
        }

        p += 1;
    }

    result * if n > 1 { n - 1 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totient() {
        assert_eq!(totient(1), 1);
        assert_eq!(totient(2), 1);
        assert_eq!(totient(3), 2);
        assert_eq!(totient(4), 2);
        assert_eq!(totient(5), 4);
        assert_eq!(totient(6), 2);
        assert_eq!(totient(7), 6);
        assert_eq!(totient(8), 4);
        assert_eq!(totient(9), 6);
        assert_eq!(totient(10), 4);
        assert_eq!(totient(11), 10);
        assert_eq!(totient(12), 4);
        assert_eq!(totient(13), 12);
        assert_eq!(totient(14), 6);
        assert_eq!(totient(15), 8);
        assert_eq!(totient(16), 8);
        assert_eq!(totient(17), 16);
        assert_eq!(totient(18), 6);
        assert_eq!(totient(19), 18);
        assert_eq!(totient(20), 8);
    }

    #[test]
    fn test_all() {
        for n in 1..=1024 {
            assert_ne!(totient(n), 0);
        }
    }
}
