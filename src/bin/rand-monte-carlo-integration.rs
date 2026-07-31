fn main() {
    let pi: Function = |x| 4.0 / (1.0 + x * x);

    let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);
    let mut trials: u64 = 256;

    for _ in 0..23 {
        println!("1st {} Estimated value of pi: {:?}", trials, integrate(&mut lcg, pi, (0.0, 1.0), trials));
        println!("2nd {} Estimated value of pi: {:?}", trials, integrate(&mut lcg, pi, (0.0, 1.0), trials));

        trials *= 2;
    }
}

type Function = fn(f64) -> f64;

fn integrate(lcg: &mut LCG<u64>, f: Function, interval: (f64, f64), n: u64) -> Option<f64> {
    if interval.0 > interval.1 || n == 0 {
        return None;
    }

    let mut sum: f64 = 0.0;
    let width = interval.1 - interval.0;
    let scale = u64::MAX as f64;

    for _ in 0..n {
        let x = lcg.next() as f64;
        let x = x / scale * width + interval.0;

        sum += f(x);
    }

    return Some((interval.1 - interval.0) * sum / (n as f64));
}

trait WrappingOps {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

impl WrappingOps for u64 {
    fn wrapping_add(self, rhs: Self) -> Self {
        self.wrapping_add(rhs)
    }

    fn wrapping_mul(self, rhs: Self) -> Self {
        self.wrapping_mul(rhs)
    }
}

struct LCG<T>
where
    T: Copy + WrappingOps,
{
    seed: T,
    a: T,
    c: T,
}

impl LCG<u64> {
    fn u64(seed: u64, a: u64, c: u64) -> Self {
        LCG { seed, a, c }
    }
}

impl<T> LCG<T>
where
    T: Copy + WrappingOps,
{
    fn next(&mut self) -> T {
        self.seed = self.seed.wrapping_mul(self.a);
        self.seed = self.seed.wrapping_add(self.c);

        return self.seed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{E, PI};

    #[test]
    fn test_integrate_x_squared() {
        let f: Function = |x| x * x;
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        let result = integrate(&mut lcg, f, (0.0, 1.0), 10000);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!((result - 1.0 / 3.0).abs() < 1e-2);
    }

    #[test]
    fn test_integrate_x_cubed() {
        let f: Function = |x| x * x * x;
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        let result = integrate(&mut lcg, f, (0.0, 1.0), 10000);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!((result - 1.0 / 4.0).abs() < 1e-2);
    }

    #[test]
    fn can_integrate_sin() {
        let f: Function = |x| x.sin();
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        let result = integrate(&mut lcg, f, (0.0, PI), 10000);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!((result - 2.0).abs() < 1e-2);
    }

    #[test]
    fn can_integrate_pi() {
        let f: Function = |x| 4.0 / (1.0 + x * x);
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        let result = integrate(&mut lcg, f, (0.0, 1.0), 10000);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!((result - PI).abs() < 1e-2);
    }

    #[test]
    fn can_integrate_e() {
        let f: Function = |x| E.powf(-x * x);
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        let result = integrate(&mut lcg, f, (0.0, 1.0), 100000);
        assert!(result.is_some());

        let result = result.unwrap();
        assert!((result - 0.746824132812427).abs() < 1e-2);
    }

    #[test]
    fn edge_invalid_interval() {
        let f: Function = |x| x * x;
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        assert_eq!(integrate(&mut lcg, f, (1.0, 0.0), 10000), None);
    }

    #[test]
    fn edge_zero_interval() {
        let f: Function = |x| x * x;
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        assert_eq!(integrate(&mut lcg, f, (1.0, 1.0), 10000), Some(0.0));
    }

    #[test]
    fn edge_zero_trials() {
        let f: Function = |x| x * x;
        let mut lcg = LCG::u64(42, 0x5d588b65, 0x1b0cb175);

        assert_eq!(integrate(&mut lcg, f, (0.0, 1.0), 0), None);
    }
}
