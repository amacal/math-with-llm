fn main() {
    let mut rnd8 = LCG::u8(0, 5, 1);
    let mut rnd16 = LCG::u16(0, 5, 1);
    let mut rnd64 = LCG::u64(0, 5, 1);

    for _ in 0..100 {
        println!("{:02x} {:04x} {:016x}", rnd8.next(), rnd16.next(), rnd64.next());
    }
}

struct LCG<const T: u64> {
    seed: u64,
    a: u64,
    c: u64,
}

impl LCG<256> {
    fn u8(seed: u8, a: u8, c: u8) -> Self {
        LCG { seed: seed as u64, a: a as u64, c: c as u64 }
    }
}

impl LCG<65536> {
    fn u16(seed: u16, a: u16, c: u16) -> Self {
        LCG { seed: seed as u64, a: a as u64, c: c as u64 }
    }
}

impl LCG<0> {
    fn u64(seed: u64, a: u64, c: u64) -> Self {
        LCG { seed, a, c }
    }
}

impl<const T: u64> LCG<T> {
    fn next(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(self.a);
        self.seed = self.seed.wrapping_add(self.c);

        if T != 0 {
            self.seed = self.seed % T;
        }

        return self.seed;
    }
}

#[cfg(test)]
mod tests {
    use crate::LCG;


    fn u3(seed: u8, a: u8, c: u8) -> LCG<8> {
        LCG { seed: seed as u64, a: a as u64, c: c as u64 }
    }

    #[test]
    fn test_lcg_5_1_8() {
        let mut rnd = u3(0, 5, 1);

        assert_eq!(rnd.next(), 1);
        assert_eq!(rnd.next(), 6);
        assert_eq!(rnd.next(), 7);
        assert_eq!(rnd.next(), 4);
        assert_eq!(rnd.next(), 5);
        assert_eq!(rnd.next(), 2);
        assert_eq!(rnd.next(), 3);
        assert_eq!(rnd.next(), 0);
        assert_eq!(rnd.next(), 1);
    }

    #[test]
    fn test_lcg_3_1_8() {
        let mut rnd = u3(0, 3, 1);

        assert_eq!(rnd.next(), 1);
        assert_eq!(rnd.next(), 4);
        assert_eq!(rnd.next(), 5);
        assert_eq!(rnd.next(), 0);
        assert_eq!(rnd.next(), 1);
    }

    #[test]
    fn test_lcg_1_1_8() {
        let mut rnd = u3(0, 1, 1);

        assert_eq!(rnd.next(), 1);
        assert_eq!(rnd.next(), 2);
        assert_eq!(rnd.next(), 3);
        assert_eq!(rnd.next(), 4);
        assert_eq!(rnd.next(), 5);
        assert_eq!(rnd.next(), 6);
        assert_eq!(rnd.next(), 7);
        assert_eq!(rnd.next(), 0);
        assert_eq!(rnd.next(), 1);
    }

    #[test]
    fn test_lcg_7_1_8() {
        let mut rnd = u3(0, 7, 1);

        assert_eq!(rnd.next(), 1);
        assert_eq!(rnd.next(), 0);
        assert_eq!(rnd.next(), 1);
    }

    #[test]
    fn test_lcg_5_2_8() {
        let mut rnd = u3(0, 5, 2);

        assert_eq!(rnd.next(), 2);
        assert_eq!(rnd.next(), 4);
        assert_eq!(rnd.next(), 6);
        assert_eq!(rnd.next(), 0);
        assert_eq!(rnd.next(), 2);
    }

    #[test]
    fn test_lcg_5_19_256() {
        let mut rnd = super::LCG::u8(0, 5, 19);
        let first = rnd.next();

        for _ in 0..255 {
            let next = rnd.next();
            assert_ne!(first, next);
        }

        assert_eq!(first, rnd.next());
    }

    #[test]
    fn test_lcg_5_19_65536() {
        let mut rnd = super::LCG::u16(0, 5, 19);
        let first = rnd.next();

        for _ in 0..65535 {
            let next = rnd.next();
            assert_ne!(first, next);
        }

        assert_eq!(first, rnd.next());
    }
}
