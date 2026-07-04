use std::cmp::min;

fn main() {
    let a = [1, 2, 3, 0, 0, 0, 0, 0];
    let b = [2, 3, 1, 0, 0, 0, 0, 0];

    println!("Convolution of {:?} and {:?} is {:?}", a, b, convolution_naive(&a, &b));
    println!("Convolution of {:?} and {:?} is {:?}", a, b, convolution_ntt(&a, &b));
}

fn convolution_naive<const T: usize>(a: &[u64; T], b: &[u64; T]) -> Option<Vec<u64>> {
    if T == 0 {
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

fn convolution_ntt(a: &[u64; 8], b: &[u64; 8]) -> [u64; 16] {
    const P_FORWARD: u64 = 17;
    const P_INVERSE: u64 = 16;

    const ROOTS_FORWARD: [u64; 16] = [1, 3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6];
    const ROOTS_INVERSE: [u64; 16] = [1, 6, 2, 12, 4, 14, 8, 9, 16, 11, 15, 5, 13, 10, 3, 7];

    let mut a2 = [0u64; 16];
    let mut b2 = [0u64; 16];

    for i in 0..8 {
        a2[i] = a[i];
        b2[i] = b[i];
    }

    ntt(&mut a2, 0, 1, &ROOTS_FORWARD, P_FORWARD);
    ntt(&mut b2, 0, 1, &ROOTS_FORWARD, P_FORWARD);

    let mut c2 = [0u64; 16];
    for i in 0..16 {
        c2[i] = a2[i] * b2[i] % P_FORWARD;
    }

    ntt_inverse(&mut c2, 0, 1, &ROOTS_INVERSE, P_FORWARD, P_INVERSE);
    return c2;
}

fn ntt_inverse<const T: usize>(coeffs: &mut [u64; T], offset: usize, stride: usize, roots: &[u64; T], p: u64, inv: u64) {
    ntt(coeffs, offset, stride, roots, p);

    for i in 0..coeffs.len() {
        coeffs[i] = coeffs[i] * inv % p;
    }
}

fn ntt<const T: usize>(coeffs: &mut [u64; T], offset: usize, stride: usize, roots: &[u64; T], p: u64) {
    let mut root = 0;
    let size: usize = (coeffs.len() / stride) >> 1;
    let mut scratch: [u64; T] = [0; T];

    if size == 0 {
        return;
    }

    ntt(coeffs, offset, stride << 1, roots, p);
    ntt(coeffs, offset + stride, stride << 1, roots, p);

    for i in 0..size {
        let e = coeffs[offset + i * (stride << 1)];
        let o = coeffs[offset + i * (stride << 1) + stride];
        let w = roots[root] * o % p;

        scratch[offset + i * stride] = (e + w) % p;
        scratch[offset + i * stride + size * stride] = (e + p - w) % p;

        root = (root + stride) % roots.len();
    }

    for i in 0..size {
        coeffs[offset + i * stride] = scratch[offset + i * stride];
        coeffs[offset + i * stride + size * stride] = scratch[offset + i * stride + size * stride];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolution_naive_same_length() {
        let a = [1, 2, 3, 0, 0, 0, 0, 0];
        let b = [2, 3, 1, 0, 0, 0, 0, 0];

        assert_eq!(convolution_naive(&a, &b), Some(vec![2, 7, 13, 11, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn test_convolution_naive_ntt_length() {
        let a = [1, 2, 3, 0, 0, 0, 0, 0];
        let b = [2, 3, 1, 0, 0, 0, 0, 0];

        assert_eq!(convolution_ntt(&a, &b), [2, 7, 13, 11, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_ntt_p5_forward() {
        let mut coeffs = [1, 2, 3, 1];
        let roots = [1, 2, 4, 3];

        ntt(&mut coeffs, 0, 1, &roots, 5);
        assert_eq!(coeffs, [2, 0, 1, 1]);
    }

    #[test]
    fn test_ntt_p5_inverse() {
        let mut coeffs = [2, 0, 1, 1];
        let roots = [1, 3, 4, 2];

        ntt_inverse(&mut coeffs, 0, 1, &roots, 5, 4);
        assert_eq!(coeffs, [1, 2, 3, 1]);
    }

    #[test]
    fn test_ntt_p17_forward() {
        let mut coeffs = [1, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let roots = [1, 3, 9, 10, 13, 5, 15, 11, 16, 14, 8, 7, 4, 12, 2, 6];

        ntt(&mut coeffs, 0, 1, &roots, 17);
        assert_eq!(coeffs, [7, 10, 5, 12, 11, 7, 1, 0, 1, 12, 7, 12, 2, 9, 8, 14]);
    }

    #[test]
    fn test_ntt_p17_inverse() {
        let mut coeffs = [7, 10, 5, 12, 11, 7, 1, 0, 1, 12, 7, 12, 2, 9, 8, 14];
        let roots = [1, 6, 2, 12, 4, 14, 8, 9, 16, 11, 15, 5, 13, 10, 3, 7];

        ntt_inverse(&mut coeffs, 0, 1, &roots, 17, 16);
        assert_eq!(coeffs, [1, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
