fn main() {
    let d = 61;
    let (cyce, terms) = find_terms(d).unwrap();

    let solution = pqa(Some((cyce, terms.clone()))).unwrap();
    let convergents = convergents(Some((cyce, &terms[..])), 2);

    println!("{}^2 - {} * {}^2 = 1\n", solution.0, d, solution.1);

    for (idx, &(p, q)) in convergents.unwrap().iter().enumerate() {
        let (p, q, d) = (p as i128, q as i128, d as i128);

        if idx > 0 {
            println!("{:<3}: {:<20} / {:<20} | {:<20} = {}", idx, p, q, (p * p - d * q * q), (p as f64) / (q as f64));
        }
    }
}

fn partial_quotient(m: u64, n: u64, d: u64) -> u64 {
    let mut lo = 0;
    let mut hi = n;

    while lo < hi {
        let mid = lo + (hi - lo).saturating_add(1) / 2;

        match mid.checked_mul(d) {
            None => hi = mid - 1,
            Some(v) if v <= m => lo = mid,
            Some(v) => match (v - m).checked_mul(v - m) {
                Some(vv) if vv <= n => lo = mid,
                _ => hi = mid - 1,
            },
        }
    }

    return lo;
}

fn find_terms(n: u64) -> Option<(usize, Vec<(u64, u64, u64)>)> {
    let isqrt = partial_quotient(0, n, 1);
    let mut terms = vec![(0, 1, isqrt)];

    if isqrt * isqrt == n {
        return None;
    }

    loop {
        let idx = terms.len() - 1;
        let (m, d, a) = terms[idx];

        let m_next = d * a - m;
        let d_next = (n - m_next * m_next) / d;
        let a_next = partial_quotient(m_next, n, d_next);

        if idx > 1 && terms[1] == (m_next, d_next, a_next) {
            return Some((1, terms));
        }

        terms.push((m_next, d_next, a_next));
    }
}

fn convergents(terms: Option<(usize, &[(u64, u64, u64)])>, mut cycles: u64) -> Option<Vec<(u64, u64)>> {
    let (cycle, terms) = terms?;
    let mut pq = vec![(1, 0), (terms[0].2, 1)];

    for idx in 1..cycle {
        let pk = terms[idx].2 * pq[idx].0 + pq[idx - 1].0;
        let qk = terms[idx].2 * pq[idx].1 + pq[idx - 1].1;

        pq.push((pk, qk));
    }

    while cycles > 0 {
        for idx in cycle..terms.len() {
            let prev = pq.len() - 1;
            let pk = terms[idx].2 * pq[prev].0 + pq[prev - 1].0;
            let qk = terms[idx].2 * pq[prev].1 + pq[prev - 1].1;

            pq.push((pk, qk));
        }

        cycles -= 1;
    }

    return Some(pq);
}

fn pqa(terms: Option<(usize, Vec<(u64, u64, u64)>)>) -> Option<(u64, u64)> {
    let (cycle, terms) = terms?;
    let r = terms.len() - cycle;

    let idx = if r % 2 == 0 { r - 1 } else { 2 * r - 1 };
    let cycles = if r % 2 == 0 { 1 } else { 2 };

    let convergent = convergents(Some((cycle, &terms)), cycles)?;
    let solution = convergent[cycle + idx].clone();

    return Some(solution);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_quotient_7() {
        assert_eq!(partial_quotient(0, 7, 1), 2);
        assert_eq!(partial_quotient(2, 7, 3), 1);
        assert_eq!(partial_quotient(1, 7, 2), 1);
        assert_eq!(partial_quotient(1, 7, 3), 1);
        assert_eq!(partial_quotient(2, 7, 1), 4);
    }

    #[test]
    fn test_partial_quotient_isqrt() {
        assert_eq!(partial_quotient(0, 0_999_999, 1), 999);
        assert_eq!(partial_quotient(0, 1_000_000, 1), 1000);
        assert_eq!(partial_quotient(0, 1_000_001, 1), 1000);
        assert_eq!(partial_quotient(0, 1_000_002, 1), 1000);

        assert_eq!(partial_quotient(0, 1_048_575, 1), 1023);
        assert_eq!(partial_quotient(0, 1_048_576, 1), 1024);
        assert_eq!(partial_quotient(0, 1_048_577, 1), 1024);
        assert_eq!(partial_quotient(0, 1_048_578, 1), 1024);
        assert_eq!(partial_quotient(0, 1_048_579, 1), 1024);
    }

    #[test]
    fn test_partial_quotient_isqrt_perfect_squares() {
        assert_eq!(partial_quotient(0, 1, 1), 1);
        assert_eq!(partial_quotient(0, 4, 1), 2);
        assert_eq!(partial_quotient(0, 9, 1), 3);
        assert_eq!(partial_quotient(0, 16, 1), 4);
        assert_eq!(partial_quotient(0, 25, 1), 5);
    }

    #[test]
    fn test_find_terms_7() {
        let (cycle, terms) = find_terms(7).unwrap();

        assert_eq!(cycle, 1);
        assert_eq!(terms.len(), 5);

        assert_eq!(terms[0], (0, 1, 2));
        assert_eq!(terms[1], (2, 3, 1));
        assert_eq!(terms[2], (1, 2, 1));
        assert_eq!(terms[3], (1, 3, 1));
        assert_eq!(terms[4], (2, 1, 4));
    }

    #[test]
    fn test_find_terms_perfect_squares() {
        assert_eq!(find_terms(1), None);
        assert_eq!(find_terms(4), None);
        assert_eq!(find_terms(9), None);
        assert_eq!(find_terms(16), None);
        assert_eq!(find_terms(25), None);
    }

    #[test]
    fn test_pqa() {
        assert_eq!(pqa(find_terms(2)), Some((3, 2)));
        assert_eq!(pqa(find_terms(3)), Some((2, 1)));
        assert_eq!(pqa(find_terms(5)), Some((9, 4)));
        assert_eq!(pqa(find_terms(6)), Some((5, 2)));
        assert_eq!(pqa(find_terms(7)), Some((8, 3)));
        assert_eq!(pqa(find_terms(13)), Some((649, 180)));
        assert_eq!(pqa(find_terms(61)), Some((1766319049, 226153980)));
    }

    #[test]
    fn test_pqa_perfect_sqaures() {
        assert_eq!(pqa(find_terms(1)), None);
        assert_eq!(pqa(find_terms(4)), None);
        assert_eq!(pqa(find_terms(9)), None);
        assert_eq!(pqa(find_terms(16)), None);
        assert_eq!(pqa(find_terms(25)), None);
    }
}
