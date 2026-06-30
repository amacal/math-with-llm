use std::cmp::min;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    println!("Convolution of {:?} and {:?} is {:?}", a, b, convolution_naive(&a, &b));
}

fn convolution_naive(a: &[i64], b: &[i64]) -> Option<Vec<i64>> {
    if a.is_empty() || b.is_empty() {
        return None;
    }

    let (n, m) = (a.len(), b.len());
    let mut result = vec![0; m + n - 1];

    for k in 0..result.len() {
        let start = (1 + k).saturating_sub(m) as usize;
        let end = min(k, n.saturating_sub(1)) as usize;

        for i in start..=end {
            result[k] += a[i] * b[k - i];
        }
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolution_naive_same_length() {
        let a = vec![1, 2, 3];
        let b = vec![4, 5, 6];

        assert_eq!(convolution_naive(&a, &b), Some(vec![4, 13, 28, 27, 18]));
    }

    #[test]
    fn test_convolution_naive_diff_length() {
        let a = vec![1, 2];
        let b = vec![4, 5, 6];

        assert_eq!(convolution_naive(&a, &b), Some(vec![4, 13, 16, 12]));
        assert_eq!(convolution_naive(&b, &a), Some(vec![4, 13, 16, 12]));
    }

    #[test]
    fn test_convolution_naive_empty() {
        let a = vec![];
        let b = vec![4, 5, 6];

        assert_eq!(convolution_naive(&a, &b), None);
        assert_eq!(convolution_naive(&b, &a), None);
        assert_eq!(convolution_naive(&a, &a), None);
    }
}
