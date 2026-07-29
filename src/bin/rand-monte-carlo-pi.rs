fn main() {
    let mut lcg = LCG::u32(42, 0x5d588b65, 0x1b0cb175);
    let mut trials: u32 = 256;

    for _ in 0..23 {
        println!("1st {} Estimated value of pi: {}", trials, compute_pi(&mut lcg, trials));
        println!("2nd {} Estimated value of pi: {}", trials, compute_pi(&mut lcg, trials));

        trials *= 2;
    }
}

fn compute_pi(lcg: &mut LCG<u32>, n: u32) -> f64 {
    let mut inside: u64 = 0;
    let mut outside: u64 = 0;

    let n = if n == 0 { 1 } else { n };

    for _ in 0..n {
        let x = lcg.next() as u64;
        let y = lcg.next() as u64;

        let x2 = x * x;
        let y2 = y * y;

        match x2.checked_add(y2) {
            Some(_) => {
                inside += 1;
            }
            None => {
                outside += 1;
            }
        }
    }

    return (inside as f64) / ((inside + outside) as f64) * 4.0;
}

trait WrappingOps {
    fn wrapping_add(self, rhs: Self) -> Self;
    fn wrapping_mul(self, rhs: Self) -> Self;
}

impl WrappingOps for u32 {
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

impl LCG<u32> {
    fn u32(seed: u32, a: u32, c: u32) -> Self {
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

    #[test]
    fn can_compute_pi_256() {
        let mut lcg = LCG::u32(42, 0x5d588b65, 0x1b0cb175);
        let pi = compute_pi(&mut lcg, 256);

        assert!((pi - std::f64::consts::PI).abs() < 0.1);
    }

    #[test]
    fn can_compute_pi_1048576() {
        let mut lcg = LCG::u32(42, 0x5d588b65, 0x1b0cb175);
        let pi = compute_pi(&mut lcg, 1048576);

        assert!((pi - std::f64::consts::PI).abs() < 0.01);
    }

    #[test]
    fn can_compute_pi_0() {
        let mut lcg = LCG::u32(42, 0x5d588b65, 0x1b0cb175);
        let pi = compute_pi(&mut lcg, 0);

        assert!(pi >= 0.0);
    }
}
