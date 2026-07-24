fn main() {
    println!("Wilson's Theorem Test for 65537: {:?}", wilson(65537));
}

fn wilson(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut factorial: u128 = 1;

    for i in 2..n as u128 {
        factorial = (factorial * i) % n as u128;

        if factorial == 0 {
            return false;
        }
    }

    return (factorial + 1) % n as u128 == 0;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_primes() {
        assert_eq!(super::wilson(2), true);
        assert_eq!(super::wilson(3), true);
        assert_eq!(super::wilson(5), true);
        assert_eq!(super::wilson(7), true);
        assert_eq!(super::wilson(11), true);
        assert_eq!(super::wilson(13), true);
        assert_eq!(super::wilson(17), true);
        assert_eq!(super::wilson(19), true);
        assert_eq!(super::wilson(23), true);
        assert_eq!(super::wilson(29), true);
        assert_eq!(super::wilson(31), true);
        assert_eq!(super::wilson(37), true);
        assert_eq!(super::wilson(41), true);
        assert_eq!(super::wilson(43), true);
        assert_eq!(super::wilson(47), true);
    }

    #[test]
    fn test_composites() {
        assert_eq!(super::wilson(4), false);
        assert_eq!(super::wilson(6), false);
        assert_eq!(super::wilson(8), false);
        assert_eq!(super::wilson(9), false);
        assert_eq!(super::wilson(10), false);
        assert_eq!(super::wilson(12), false);
        assert_eq!(super::wilson(14), false);
        assert_eq!(super::wilson(15), false);
        assert_eq!(super::wilson(16), false);
        assert_eq!(super::wilson(18), false);
        assert_eq!(super::wilson(20), false);
        assert_eq!(super::wilson(21), false);
        assert_eq!(super::wilson(22), false);
        assert_eq!(super::wilson(24), false);
        assert_eq!(super::wilson(25), false);
        assert_eq!(super::wilson(26), false);
        assert_eq!(super::wilson(27), false);
        assert_eq!(super::wilson(28), false);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(super::wilson(0), false);
        assert_eq!(super::wilson(1), false);
    }
}
