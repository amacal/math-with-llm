use std::{
    cmp::{Ordering, max},
    ops::{Add, Div, Mul, Sub},
};

fn main() {
    let mut lcg = LCG::u64(0, 5, 1);
    let limbs = 16;

    for _ in 0..256 {
        lcg.next();
    }

    let base = BigNumber::random(&mut lcg, limbs);
    let exp = BigNumber::random(&mut lcg, limbs);
    let modulus = BigNumber::random(&mut lcg, limbs + 1);

    println!("base: {:?}", base);
    println!("exp: {:?}", exp);
    println!("modulus: {:?}", modulus);
    println!("mod_exp: {:?}", mod_exp(base, exp, modulus));
}

struct LCG<const T: u64> {
    seed: u64,
    a: u64,
    c: u64,
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

#[derive(Debug, Clone)]
struct BigNumber {
    data: Vec<u64>,
}

impl BigNumber {
    fn new(data: Vec<u64>) -> Self {
        BigNumber { data }
    }

    fn from(data: &[u64]) -> Self {
        return BigNumber::new(data.to_vec());
    }

    fn one() -> Self {
        return BigNumber::new(vec![1]);
    }

    fn random(lcg: &mut LCG<0>, limbs: usize) -> Self {
        let mut data = Vec::with_capacity(limbs);

        for _ in 0..limbs - 1 {
            data.push(lcg.next());
        }

        while data.len() < limbs {
            let val = lcg.next();
            if val != 0 {
                data.push(val);
            }
        }

        return BigNumber::new(data);
    }

    fn at(&self, idx: usize) -> u64 {
        return self.data[idx];
    }

    fn len(&self) -> usize {
        return self.data.len();
    }

    fn is_zero(&self) -> bool {
        return self.data.len() == 1 && self.data[0] == 0;
    }

    fn extend(&self, limbs: usize) -> Self {
        let mut data = self.data.clone();
        data.resize(data.len() + limbs, 0);
        return BigNumber::new(data);
    }

    fn shift(&self, limbs: usize) -> Self {
        let mut data = vec![0; self.data.len() + limbs];

        for (idx, &val) in self.data.iter().enumerate() {
            data[idx + limbs] = val;
        }

        return BigNumber::new(data);
    }
}

impl PartialEq for BigNumber {
    fn eq(&self, other: &Self) -> bool {
        if self.data.len() != other.data.len() {
            return false;
        }

        for idx in (0..self.data.len()).rev() {
            if self.data[idx] != other.data[idx] {
                return false;
            }
        }

        return true;
    }
}

impl PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for BigNumber {}

impl Ord for BigNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.data.len().cmp(&other.data.len()) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }

        for idx in (0..self.data.len()).rev() {
            match self.data[idx].cmp(&other.data[idx]) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            }
        }

        return Ordering::Equal;
    }
}

impl Add for &BigNumber {
    type Output = BigNumber;

    fn add(self, other: Self) -> BigNumber {
        let size = max(self.data.len(), other.data.len());
        let mut data = Vec::with_capacity(size + 1);
        let mut carry = 0;

        for idx in 0..size {
            let lhs = match self.data.get(idx) {
                Some(&val) => val,
                None => 0,
            };

            let rhs = match other.data.get(idx) {
                Some(&val) => val,
                None => 0,
            };

            let (sum1, carry1) = lhs.overflowing_add(rhs);
            let (sum2, carry2) = sum1.overflowing_add(carry);

            data.push(sum2);
            carry = (carry1 as u64) + (carry2 as u64);
        }

        if carry > 0 {
            data.push(carry);
        }

        return BigNumber::new(data);
    }
}

impl Sub for &BigNumber {
    type Output = Option<BigNumber>;

    fn sub(self, other: Self) -> Option<BigNumber> {
        let size = max(self.data.len(), other.data.len());
        let mut data = Vec::with_capacity(size);
        let mut borrow = 0;

        for idx in 0..size {
            let lhs = match self.data.get(idx) {
                Some(&val) => val,
                None => 0,
            };

            let rhs = match other.data.get(idx) {
                Some(&val) => val,
                None => 0,
            };

            let (sub1, borrow1) = lhs.overflowing_sub(rhs);
            let (sub2, borrow2) = sub1.overflowing_sub(borrow);

            data.push(sub2);
            borrow = (borrow1 as u64) + (borrow2 as u64);
        }

        if borrow > 0 {
            return None;
        }

        while data.len() > 1 && data.last() == Some(&0) {
            data.pop();
        }

        return Some(BigNumber::new(data));
    }
}

impl Mul<&BigNumber> for &BigNumber {
    type Output = BigNumber;

    fn mul(self, other: &BigNumber) -> BigNumber {
        let size = self.data.len() + other.data.len();
        let mut data = vec![0; size];
        let mut carries = vec![0; size + 1];

        for (i, &lhs) in self.data.iter().enumerate() {
            for (j, &rhs) in other.data.iter().enumerate() {
                let (low, mut high) = mul128(lhs, rhs);

                let (sum1, carry1) = low.overflowing_add(data[i + j]);
                data[i + j] = sum1;

                // high has always a place for a carry
                high = high + (carry1 as u64);

                let (sum2, carry2) = high.overflowing_add(data[i + j + 1]);
                data[i + j + 1] = sum2;

                // safely add the carry to the next position
                carries[i + j + 2] += carry2 as u64;
            }
        }

        for idx in 0..data.len() {
            let (sum, carry) = data[idx].overflowing_add(carries[idx]);

            data[idx] = sum;
            carries[idx + 1] += carry as u64;
        }

        while data.len() > 1 && data.last() == Some(&0) {
            data.pop();
        }

        return BigNumber::new(data);
    }
}

fn mul128(lhs: u64, rhs: u64) -> (u64, u64) {
    let (lhigh, llow) = (lhs >> 32, lhs & 0xffffffff);
    let (rhigh, rlow) = (rhs >> 32, rhs & 0xffffffff);

    let ll = llow * rlow;
    let lh = llow * rhigh;
    let hl = lhigh * rlow;
    let hh = lhigh * rhigh;

    // combine naive results
    let (mut high, mut low) = (hh, ll);

    // append lh term
    let (lh1, lh2) = (lh >> 32, lh << 32);
    let (sum1, carry1) = low.overflowing_add(lh2);

    // append hl term
    let (hl1, hl2) = (hl >> 32, hl << 32);
    let (sum2, carry2) = sum1.overflowing_add(hl2);

    // low is the sum of ll, lh2, and hl2
    low = sum2;

    // high is the sum of hh, lh1, hl1, and the carries from low
    high = high + lh1 + hl1 + (carry1 as u64) + (carry2 as u64);

    return (low, high);
}

impl Div for &BigNumber {
    type Output = Option<(BigNumber, BigNumber)>;

    fn div(self, other: Self) -> Self::Output {
        if other.is_zero() {
            return None;
        }

        if self < other {
            return Some((BigNumber::new(vec![0]), self.clone()));
        }

        let mut data = vec![];
        let leader = other.at(other.len() - 1) as u128;

        let base = u64::MAX as u128 + 1;
        let q = (base / (leader + 1)) as u64;

        let dividend = (self * q).extend(1);
        let divisor = other * q;
        let leader = divisor.at(divisor.len() - 1);

        let off = dividend.len() - divisor.len() - 1;
        let mut window = BigNumber::from(&dividend.data[off..]);

        for idx in 0..off + 1 {
            let hi = window.at(window.len() - 1);
            let lo = window.at(window.len() - 2);

            let val = ((hi as u128) << 64) | (lo as u128);
            let q = val / (leader as u128);

            let mut q = if q > u64::MAX as u128 { u64::MAX } else { q as u64 };
            let mut candidate = &divisor * q;

            while candidate > window {
                candidate = (&candidate - &divisor)?;
                q -= 1;
            }

            data.push(q);
            window = (&window - &candidate)?;

            if idx < off {
                window = window.shift(1);
                window = &window + &BigNumber::from(&dividend.data[off - idx - 1..off - idx]);
            }
        }

        let mut data = data.into_iter().rev().collect::<Vec<u64>>();
        while data.len() > 1 && data.last() == Some(&0) {
            data.pop();
        }

        let q = BigNumber::new(vec![q]);
        let remainder = match &window / &q {
            Some((quotient, _)) => quotient,
            None => return None,
        };

        return Some((BigNumber::new(data), remainder));
    }
}

impl Mul<u64> for &BigNumber {
    type Output = BigNumber;

    fn mul(self, rhs: u64) -> Self::Output {
        let size = self.data.len();
        let mut data = vec![0; size + 1];
        let mut carries = vec![0; size + 2];

        for (idx, &leg) in self.data.iter().enumerate() {
            let (low, high) = mul128(leg, rhs);
            let (sum1, carry1) = low.overflowing_add(data[idx]);
            let (sum2, carry2) = high.overflowing_add(data[idx + 1]);

            data[idx] = sum1;
            data[idx + 1] = sum2;

            carries[idx + 1] += (carry1 as u64) + (carry2 as u64);
        }

        for idx in 0..data.len() {
            let (sum, carry) = data[idx].overflowing_add(carries[idx]);

            data[idx] = sum;
            carries[idx + 1] += carry as u64;
        }

        while data.len() > 1 && data.last() == Some(&0) {
            data.pop();
        }

        return BigNumber::new(data);
    }
}

fn mod_exp(base: BigNumber, exp: BigNumber, modulus: BigNumber) -> Option<BigNumber> {
    if modulus.is_zero() {
        return None;
    }

    let mut power = base.div(&modulus)?.1;
    let mut result = BigNumber::one().div(&modulus)?.1;

    for val in exp.data {
        for bit in 0..64 {
            if (val >> bit) & 0x01 == 0x01 {
                let tmp = &result * &power;
                result = tmp.div(&modulus)?.1;
            }

            let tmp = &power * &power;
            power = tmp.div(&modulus)?.1;
        }
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        assert_eq!(
            mod_exp(BigNumber::from(&[2]), BigNumber::from(&[5]), BigNumber::from(&[17])),
            Some(BigNumber::from(&[32]).div(&BigNumber::from(&[17])).unwrap().1)
        );

        assert_eq!(
            mod_exp(BigNumber::from(&[3]), BigNumber::from(&[4]), BigNumber::from(&[7])),
            Some(BigNumber::from(&[81]).div(&BigNumber::from(&[7])).unwrap().1)
        );

        assert_eq!(
            mod_exp(BigNumber::from(&[5]), BigNumber::from(&[3]), BigNumber::from(&[13])),
            Some(BigNumber::from(&[125]).div(&BigNumber::from(&[13])).unwrap().1)
        );

        assert_eq!(
            mod_exp(BigNumber::from(&[10]), BigNumber::from(&[6]), BigNumber::from(&[11])),
            Some(BigNumber::from(&[1000000]).div(&BigNumber::from(&[11])).unwrap().1)
        );

        assert_eq!(
            mod_exp(BigNumber::from(&[7]), BigNumber::from(&[8]), BigNumber::from(&[19])),
            Some(BigNumber::from(&[5764801]).div(&BigNumber::from(&[19])).unwrap().1)
        );
    }

    #[test]
    fn test_mod_zero() {
        assert_eq!(mod_exp(BigNumber::from(&[0]), BigNumber::from(&[5]), BigNumber::from(&[17])), Some(BigNumber::from(&[0])));
        assert_eq!(mod_exp(BigNumber::from(&[2]), BigNumber::from(&[0]), BigNumber::from(&[17])), Some(BigNumber::from(&[1])));
        assert_eq!(mod_exp(BigNumber::from(&[0]), BigNumber::from(&[0]), BigNumber::from(&[17])), Some(BigNumber::from(&[1])));
        assert_eq!(mod_exp(BigNumber::from(&[2]), BigNumber::from(&[0]), BigNumber::from(&[1])), Some(BigNumber::from(&[0])));
    }

    #[test]
    fn test_mod_one() {
        assert_eq!(mod_exp(BigNumber::from(&[2]), BigNumber::from(&[5]), BigNumber::from(&[1])), Some(BigNumber::from(&[0])));
        assert_eq!(mod_exp(BigNumber::from(&[3]), BigNumber::from(&[4]), BigNumber::from(&[1])), Some(BigNumber::from(&[0])));
        assert_eq!(mod_exp(BigNumber::from(&[5]), BigNumber::from(&[3]), BigNumber::from(&[1])), Some(BigNumber::from(&[0])));
    }

    #[test]
    fn test_mod_exp_32_limbs() {
        #[rustfmt::skip]
        let base = BigNumber::new(vec![
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210,
            0x0f0f_f0f0_aaaa_5555,
            0xdead_beef_cafe_babe,
            0x1357_9bdf_2468_ace0,
            0x89ab_cdef_0123_4567,
            0x55aa_55aa_aa55_aa55,
            0xc001_d00d_f00d_beef,
            0x1111_2222_3333_4444,
            0x5555_6666_7777_8888,
            0x9999_aaaa_bbbb_cccc,
            0xdddd_eeee_ffff_0001,
            0x6a09_e667_f3bc_c908,
            0xbb67_ae85_84ca_a73b,
            0x3c6e_f372_fe94_f82b,
            0xa54f_f53a_5f1d_36f1,
            0x510e_527f_ade6_82d1,
            0x9b05_688c_2b3e_6c1f,
            0x1f83_d9ab_fb41_bd6b,
            0x5be0_cd19_137e_2179,
            0xa5a5_a5a5_5a5a_5a5a,
            0xf0f0_0f0f_cccc_3333,
            0x0123_0123_4567_4567,
            0x89ab_89ab_cdef_cdef,
            0x0011_2233_4455_6677,
            0x8899_aabb_ccdd_eeff,
            0x1020_3040_5060_7080,
            0x90a0_b0c0_d0e0_f001,
            0x7fff_ffff_ffff_ffff,
            0x8000_0000_0000_0001,
            0xabcd_ef01_2345_6789,
            0xfedc_ba98_7654_3211,
        ]);

        #[rustfmt::skip]
        let exp = BigNumber::new(vec![
            0x0000_0000_0000_0001,
            0x0000_0000_0000_0003,
            0xaaaa_aaaa_5555_5555,
            0x1234_5678_9abc_def0,
            0x0f1e_2d3c_4b5a_6978,
            0x8877_6655_4433_2211,
            0xdead_beef_0000_0001,
            0x0001_0002_0004_0008,
            0x1357_2468_1357_2468,
            0xfedc_0123_ba98_4567,
            0x3141_5926_5358_9793,
            0x2384_6264_3383_2795,
            0x2718_2818_2845_9045,
            0x2353_6028_7471_3526,
            0x0101_0101_0101_0101,
            0x8080_8080_8080_8080,
            0x7a6b_5c4d_3e2f_1a09,
            0x89ab_cdef_0123_4567,
            0x7654_3210_fedc_ba98,
            0x1122_3344_5566_7788,
            0x99aa_bbcc_ddee_ff00,
            0xffff_0000_ffff_0000,
            0x0000_ffff_0000_ffff,
            0xa5a5_a5a5_a5a5_a5a5,
            0x5a5a_5a5a_5a5a_5a5a,
            0x1234_1234_1234_1234,
            0x4321_4321_4321_4321,
            0xff00_ff00_00ff_00ff,
            0x003f_00fc_03f0_0fc0,
            0xc0ff_ee00_faca_de00,
            0xdead_beef_cafe_babe,
            0xcafe_babe_face_feed,
        ]);

        #[rustfmt::skip]
        let modulus = BigNumber::new(vec![
            0xffff_ffff_ffff_ffc5,
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210,
            0x3141_5926_5358_9793,
            0x2718_2818_2845_9045,
            0x0f0f_0f0f_f0f0_f0f0,
            0xaaaa_aaaa_aaaa_aaaa,
            0x5555_5555_5555_5555,
            0x0123_0123_0123_0123,
            0x4567_4567_4567_4567,
            0x89ab_89ab_89ab_89ab,
            0xcdef_cdef_cdef_cdef,
            0x6a09_e667_f3bc_c908,
            0xbb67_ae85_84ca_a73b,
            0x3c6e_f372_fe94_f82b,
            0xa54f_f53a_5f1d_36f1,
            0x510e_527f_ade6_82d1,
            0x9b05_688c_2b3e_6c1f,
            0x1f83_d9ab_fb41_bd6b,
            0x5be0_cd19_137e_2179,
            0xdead_beef_cafe_babe,
            0xc001_d00d_f00d_beef,
            0x0bad_f00d_feed_face,
            0xabcd_ef01_2345_6789,
            0x1029_3847_56ab_cdef,
            0xfedc_ba65_4738_2910,
            0x1122_3344_5566_7788,
            0x8877_6655_4433_2211,
            0x7fff_0000_ffff_0001,
            0x8fff_1111_eeee_2222,
            0x9abc_deff_edcb_a987,
            0xd123_4567_89ab_cdef,
        ]);

        #[rustfmt::skip]
        assert_eq!(
            mod_exp(base, exp, modulus),
            Some(BigNumber::new(vec![
                0xf55d_fc7a_a0ee_fe07,
                0xd77b_545a_0db4_46ef,
                0x23af_9f30_0c3f_3872,
                0xad1f_984b_afbe_2055,
                0xe7c7_4d3f_bda4_5c2d,
                0x42f4_fd41_b866_0c19,
                0x3fdb_9d4b_874f_f18a,
                0x46ee_8af0_fa66_9f19,
                0x0bbc_bce1_5220_c84e,
                0x4984_49f4_1d9d_0a90,
                0x443c_e710_2d23_9fd6,
                0xc14a_69db_7c3c_d4a8,
                0x22cd_46cc_31bf_f986,
                0x6426_6c9a_b4ab_ab46,
                0x3605_4dce_54d1_6f4d,
                0xcb0e_5ddf_d8ca_cfb0,
                0xb7cc_643d_90ed_6b8e,
                0x75ff_dba6_456c_bca0,
                0x2381_72eb_2e6b_18a0,
                0x88ed_a3d7_71e4_c378,
                0xab89_2e03_3653_57e3,
                0xbb4f_bc37_0579_eb2d,
                0xbb62_b735_780d_f3d9,
                0x32c7_8f1f_c9cc_c083,
                0x41f8_5f92_6508_a0b8,
                0x0182_b3f1_50ff_6efa,
                0xbb82_84c5_ac55_1836,
                0x77a9_a345_0982_d46c,
                0x86ff_f7a5_9433_7536,
                0xd76d_4c25_190f_2b28,
                0x0e02_b2b9_5b80_59d1,
                0x800d_70ec_2f58_a457,
            ]))
        );
    }
}
