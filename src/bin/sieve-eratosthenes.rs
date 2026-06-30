fn main() {
    println!("Prime numbers up to 100: {:?}", sieve(100));
}

fn sieve(n: usize) -> Vec<usize> {
    let mut scratch = vec![true; n + 1];
    let mut primes = Vec::new();

    let mut multiple;
    let limit = (n as f64).sqrt() as usize;

    for p in 2..=limit {
        if scratch[p] {
            multiple = p * p;

            while multiple <= n {
                scratch[multiple] = false;
                multiple += p;
            }
        }
    }

    for p in 2..=n {
        if scratch[p] {
            primes.push(p);
        }
    }

    return primes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let primes = sieve(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_sieve_edge_cases() {
        assert_eq!(sieve(0), vec![]);
        assert_eq!(sieve(1), vec![]);
        assert_eq!(sieve(2), vec![2]);
        assert_eq!(sieve(3), vec![2, 3]);
    }
}
