fn main() {
    for (idx, &(p, q)) in sqrt2(50).unwrap().iter().enumerate() {
        println!("{}: {} / {} = {}", idx, p, q, (p as f64) / (q as f64));
    }
}

fn convergents(terms: &[u64]) -> Option<Vec<(u64, u64)>> {
    if terms.is_empty() {
        return None;
    }

    let mut pq = vec![(1, 0), (terms[0], 1)];

    for idx in 1..terms.len() {
        let pk = terms[idx] * pq[idx].0 + pq[idx - 1].0;
        let qk = terms[idx] * pq[idx].1 + pq[idx - 1].1;

        pq.push((pk, qk));
    }

    return Some(pq);
}

fn sqrt2(n: usize) -> Option<Vec<(u64, u64)>> {
    if n == 0 {
        return None;
    }

    let mut terms = vec![1];

    for _ in 1..n {
        terms.push(2);
    }

    return convergents(&terms);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convergents_17_by_7() {
        let terms = vec![2, 2, 3];
        let convergents = convergents(&terms).unwrap();

        assert_eq!(convergents[0], (1, 0));
        assert_eq!(convergents[1], (2, 1));
        assert_eq!(convergents[2], (5, 2));
        assert_eq!(convergents[3], (17, 7));
    }

    #[test]
    fn test_convergents_13_by_8() {
        let terms = vec![1, 1, 1, 1, 2];
        let convergents = convergents(&terms).unwrap();

        assert_eq!(convergents[0], (1, 0));
        assert_eq!(convergents[1], (1, 1));
        assert_eq!(convergents[2], (2, 1));
        assert_eq!(convergents[3], (3, 2));
        assert_eq!(convergents[4], (5, 3));
        assert_eq!(convergents[5], (13, 8));
    }

    #[test]
    fn test_sqrt2_short() {
        let convergents = sqrt2(5).unwrap();

        assert_eq!(convergents[0], (1, 0));
        assert_eq!(convergents[1], (1, 1));
        assert_eq!(convergents[2], (3, 2));
        assert_eq!(convergents[3], (7, 5));
        assert_eq!(convergents[4], (17, 12));
    }

    #[test]
    fn test_sqrt2_long() {
        let convergents = sqrt2(50).unwrap();

        for (idx, &(p, q)) in convergents.iter().enumerate() {
            let p = p as i128;
            let q = q as i128;

            assert_eq!(p * p - 2 * q * q, if idx % 2 == 0 { 1 } else { -1 });
        }
    }
}
