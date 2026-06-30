fn main() {
    println!("Prime numbers between 2^20 and 2^21: {:?}", sieve_segmented(1 << 20, 1 << 21));
}

fn sieve_eratosthenes(n: usize) -> Vec<usize> {
    let mut scratch = vec![true; n + 1];
    let mut primes = Vec::new();

    let mut multiple;
    let limit = (n as f64).sqrt().ceil() as usize;

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

fn sieve_segmented(l: usize, r: usize) -> Option<Vec<usize>> {
    if l > r {
        return None;
    }

    if r < 2 || l < 2 {
        return Some(vec![]);
    }

    let limit = (r as f64).sqrt().ceil() as usize;
    let eratosthenes = sieve_eratosthenes(limit);

    let mut scratch = vec![true; r - l + 1];
    let mut primes = Vec::new();

    for &p in eratosthenes.iter() {
        let mut m = (l + p - 1) / p * p;

        if m == p {
            m += p;
        }

        while m <= r {
            scratch[m - l] = false;
            m += p;
        }
    }

    for i in 0..scratch.len() {
        if scratch[i] {
            primes.push(i + l);
        }
    }

    return Some(primes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve_eratosthenes() {
        let primes = sieve_eratosthenes(30);
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_sieve_eratosthenes_edge_cases() {
        assert_eq!(sieve_eratosthenes(0), vec![]);
        assert_eq!(sieve_eratosthenes(1), vec![]);
        assert_eq!(sieve_eratosthenes(2), vec![2]);
        assert_eq!(sieve_eratosthenes(3), vec![2, 3]);
    }

    #[test]
    fn test_sieve_segmented_small() {
        let primes = sieve_segmented(10, 30);
        assert_eq!(primes, Some(vec![11, 13, 17, 19, 23, 29]));
    }

    #[test]
    fn test_sieve_segmented_large() {
        let primes = sieve_segmented(100, 200);
        assert_eq!(primes, Some(vec![101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199]));
    }

    #[test]
    fn test_sieve_segmented_single_prime() {
        assert_eq!(sieve_segmented(29, 29), Some(vec![29]));
    }

    #[test]
    fn test_sieve_segmented_edge_cases() {
        assert_eq!(sieve_segmented(0, 0), Some(vec![]));
        assert_eq!(sieve_segmented(0, 1), Some(vec![]));
        assert_eq!(sieve_segmented(1, 1), Some(vec![]));
        assert_eq!(sieve_segmented(2, 2), Some(vec![2]));
        assert_eq!(sieve_segmented(2, 3), Some(vec![2, 3]));
        assert_eq!(sieve_segmented(48, 49), Some(vec![]));
    }
}
