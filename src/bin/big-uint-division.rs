use std::{
    cmp::{Ordering, max},
    ops::{Add, Div, Mul, Sub},
};

fn main() {
    let p = BigNumber::new(vec![0xffff_ffff_ffff_ffff, 0xffff_ffff_ffff_ffff]);
    let q = BigNumber::new(vec![0x03]);

    println!("p:   {:x?}", p);
    println!("q:   {:x?}", q);

    let r = &p / &q;
    println!("p/q: {:x?}", r);
}

#[derive(Debug)]
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
    type Output = Option<BigNumber>;

    fn div(self, other: Self) -> Option<BigNumber> {
        if other.is_zero() {
            return None;
        }

        if self < other {
            return Some(BigNumber::new(vec![0]));
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

        return Some(BigNumber::new(data));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_number_equality() {
        let num1 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num2 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num3 = BigNumber::new(vec![0x03, 0x02, 0x01]);
        let num4 = BigNumber::new(vec![0x01, 0x02]);

        assert_eq!(num1, num2);
        assert_ne!(num1, num3);
        assert_ne!(num1, num4);
        assert_ne!(num3, num4);
    }

    #[test]
    fn test_big_number_ordering() {
        let num1 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num2 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num3 = BigNumber::new(vec![0x03, 0x02, 0x01]);
        let num4 = BigNumber::new(vec![0x01, 0x02]);

        assert_eq!(num1.cmp(&num2), Ordering::Equal);
        assert_eq!(num1.cmp(&num3), Ordering::Greater);
        assert_eq!(num3.cmp(&num1), Ordering::Less);
        assert_eq!(num1.cmp(&num4), Ordering::Greater);
        assert_eq!(num4.cmp(&num1), Ordering::Less);
    }

    #[test]
    fn test_big_number_addition() {
        let num1 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num2 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num3 = BigNumber::new(vec![0x03, 0x02, 0x01]);
        let num4 = BigNumber::new(vec![0x01, 0x02]);

        assert_eq!(&num1 + &num2, BigNumber::new(vec![0x02, 0x04, 0x06]));
        assert_eq!(&num1 + &num3, BigNumber::new(vec![0x04, 0x04, 0x04]));
        assert_eq!(&num1 + &num4, BigNumber::new(vec![0x02, 0x04, 0x03]));
    }

    #[test]
    fn test_big_number_addition_with_carry() {
        let num1 = BigNumber::new(vec![0xffffffffffffffff, 0xffffffffffffffff]);
        let num2 = BigNumber::new(vec![0xffffffffffffffff]);
        let num3 = BigNumber::new(vec![0x01]);

        assert_eq!(&num2 + &num3, BigNumber::new(vec![0x00, 0x01]));
        assert_eq!(&num1 + &num3, BigNumber::new(vec![0x00, 0x00, 0x01]));
        assert_eq!(&num1 + &num2, BigNumber::new(vec![0xfffffffffffffffe, 0x00, 0x01]));
    }

    #[test]
    fn test_big_number_subtraction() {
        let num1 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num2 = BigNumber::new(vec![0x01, 0x02, 0x02]);
        let num3 = BigNumber::new(vec![0x01, 0x00, 0x03]);
        let num4 = BigNumber::new(vec![0x01, 0x02]);

        assert_eq!(&num1 - &num1, Some(BigNumber::new(vec![0x00])));
        assert_eq!(&num1 - &num2, Some(BigNumber::new(vec![0x00, 0x00, 0x01])));
        assert_eq!(&num1 - &num3, Some(BigNumber::new(vec![0x00, 0x02])));
        assert_eq!(&num1 - &num4, Some(BigNumber::new(vec![0x00, 0x00, 0x03])));
    }

    #[test]
    fn test_big_number_subtraction_with_borrow() {
        let num1 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num2 = BigNumber::new(vec![0x01, 0x03, 0x02]);

        assert_eq!(&num1 - &num2, Some(BigNumber::new(vec![0x00, 0xffffffffffffffff])));
    }

    #[test]
    fn test_big_number_subtraction_negative_result() {
        let num1 = BigNumber::new(vec![0x01, 0x02, 0x03]);
        let num2 = BigNumber::new(vec![0x01, 0x02, 0x04]);

        assert_eq!(&num1 - &num2, None);
    }

    #[test]
    fn test_mul128_naive_cases() {
        assert_eq!(mul128(0x07, 0x08), (0x38, 0x00));
        assert_eq!(mul128(0x07, 0x00), (0x00, 0x00));
        assert_eq!(mul128(0x00, 0x08), (0x00, 0x00));
        assert_eq!(mul128(0xff, 0xff), (0xfe01, 0x00));
    }

    #[test]
    fn test_mul128_carry_cases() {
        assert_eq!(mul128(0xffffffffffffffff, 0xffffffffffffffff), (0x0000000000000001, 0xfffffffffffffffe));
        assert_eq!(mul128(0xffffffffffffffff, 0x0000000000000001), (0xffffffffffffffff, 0x0000000000000000));
        assert_eq!(mul128(0x0000000000000001, 0xffffffffffffffff), (0xffffffffffffffff, 0x0000000000000000));
        assert_eq!(mul128(0x8000000000000000, 0x0000000000000002), (0x0000000000000000, 0x0000000000000001));
    }

    #[test]
    fn test_big_number_naive() {
        let num1 = BigNumber::new(vec![0x07]);
        let num2 = BigNumber::new(vec![0x08]);
        let num3 = BigNumber::new(vec![0x00]);

        assert_eq!(&num1 * &num2, BigNumber::new(vec![0x38]));
        assert_eq!(&num1 * &num3, BigNumber::new(vec![0x00]));
        assert_eq!(&num3 * &num2, BigNumber::new(vec![0x00]));
    }

    #[test]
    fn test_big_number_multi() {
        let num1 = BigNumber::new(vec![0x07]);
        let num2 = BigNumber::new(vec![0x09, 0x0a]);

        assert_eq!(&num1 * &num2, BigNumber::new(vec![0x3f, 0x46]));
        assert_eq!(&num2 * &num1, BigNumber::new(vec![0x3f, 0x46]));
    }

    #[test]
    fn test_big_number_multi_carry() {
        let num1 = BigNumber::new(vec![0xffffffffffffffff]);
        let num2 = BigNumber::new(vec![0xffffffffffffffff]);

        assert_eq!(&num1 * &num2, BigNumber::new(vec![0x0000000000000001, 0xfffffffffffffffe]));
    }

    #[test]
    fn test_big_number_mul_u64_without_carry() {
        let num1 = BigNumber::new(vec![0x07, 0x08]);
        let num2 = 0x09;

        assert_eq!(&num1 * num2, BigNumber::new(vec![0x3f, 0x48]));
    }

    #[test]
    fn test_big_number_mul_u64_with_carry() {
        let num1 = BigNumber::new(vec![0xffffffffffffffff, 0x01]);
        let num2 = 0x02;

        assert_eq!(&num1 * num2, BigNumber::new(vec![0xfffffffffffffffe, 0x03]));
    }

    #[test]
    fn test_big_number_mul_u64_by_zero() {
        let num1 = BigNumber::new(vec![0x07, 0x08]);
        let num2 = 0x00;

        assert_eq!(&num1 * num2, BigNumber::new(vec![0x00]));
    }

    #[test]
    fn test_big_number_mul_u64_by_one() {
        let num1 = BigNumber::new(vec![0x07, 0x08]);
        let num2 = 0x01;

        assert_eq!(&num1 * num2, BigNumber::new(vec![0x07, 0x08]));
    }

    #[test]
    fn test_big_number_mul_u64_by_0x0d() {
        let num1 = BigNumber::new(vec![0x09d89d89d89d89d8]);
        let num2 = 0x0d;

        assert_eq!(&num1 * num2, BigNumber::new(vec![0x7ffffffffffffff8]));
    }

    #[test]
    fn test_big_number_div_by_one() {
        let num1 = BigNumber::new(vec![0x07, 0x08]);
        let num2 = BigNumber::new(vec![0x01]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x07, 0x08])));
    }

    #[test]
    fn test_big_number_div_by_thirteen() {
        let num1 = BigNumber::new(vec![0x07, 0x20]);
        let num2 = BigNumber::new(vec![0x0d]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x7627627627627627, 0x02])));
    }

    #[test]
    fn test_big_number_div_by_zero() {
        let num1 = BigNumber::new(vec![0x07, 0x08]);
        let num2 = BigNumber::new(vec![0x00]);

        assert_eq!(&num1 / &num2, None);
    }

    #[test]
    fn test_big_number_is_zero() {
        let num1 = BigNumber::new(vec![0x00]);
        let num2 = BigNumber::new(vec![0x07]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x00])));
    }

    #[test]
    fn test_div_single_limb_exact() {
        let num1 = BigNumber::new(vec![0xffff_ffff_ffff_ffff]);
        let num2 = BigNumber::new(vec![0x03]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x5555_5555_5555_5555])));
    }

    #[test]
    fn test_div_single_limb_with_cross_limb_carry() {
        let num1 = BigNumber::new(vec![0x00, 0x01]);
        let num2 = BigNumber::new(vec![0x02]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x8000_0000_0000_0000])));
    }

    #[test]
    fn test_div_two_limbs_by_single_limb() {
        let num1 = BigNumber::new(vec![0xffff_ffff_ffff_ffff, 0xffff_ffff_ffff_ffff]);
        let num2 = BigNumber::new(vec![0x03]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x5555_5555_5555_5555, 0x5555_5555_5555_5555,])));
    }

    #[test]
    fn test_div_by_power_of_two() {
        let num1 = BigNumber::new(vec![0x0123_4567_89ab_cdef, 0xfedc_ba98_7654_3210]);
        let num2 = BigNumber::new(vec![0x10]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x0012_3456_789a_bcde, 0x0fed_cba9_8765_4321,])));
    }

    #[test]
    fn test_div_number_by_itself() {
        let num1 = BigNumber::new(vec![0xdead_beef_cafe_babe, 0x0123_4567_89ab_cdef, 0x42]);

        assert_eq!(&num1 / &num1, Some(BigNumber::new(vec![1])));
    }

    #[test]
    fn test_div_smaller_by_larger() {
        let num1 = BigNumber::new(vec![0xffff_ffff_ffff_ffff]);
        let num2 = BigNumber::new(vec![0x00, 0x01]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0])));
    }

    #[test]
    fn test_div_larger_by_almost_same_number() {
        let num1 = BigNumber::new(vec![0xffff_ffff_ffff_ffff, 0xffff_ffff_ffff_ffff]);
        let num2 = BigNumber::new(vec![0xffff_ffff_ffff_fffe, 0xffff_ffff_ffff_ffff]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![1])));
    }

    #[test]
    fn test_div_exact_multi_limb() {
        let num1 = BigNumber::new(vec![0x0d, 0x0d]);
        let num2 = BigNumber::new(vec![0x0d]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![0x01, 0x01])));
    }

    #[test]
    fn test_div_max_three_limbs_by_max_limb() {
        let num1 = BigNumber::new(vec![u64::MAX, u64::MAX, u64::MAX]);
        let num2 = BigNumber::new(vec![u64::MAX]);

        assert_eq!(&num1 / &num2, Some(BigNumber::new(vec![1, 1, 1])));
    }

    #[test]
    fn test_div_seven_limbs_by_four_limbs() {
        #[rustfmt::skip]
        let num1 = BigNumber::new(vec![
            0x0123_4567_89ab_cdef,
            0xfedc_ba98_7654_3210,
            0x1111_1111_1111_1111,
            0x2222_2222_2222_2222,
            0x3333_3333_3333_3333,
            0x4444_4444_4444_4444,
            0x5555_5555_5555_5555,
        ]);

        #[rustfmt::skip]
        let num2 = BigNumber::new(vec![
            0xdead_beef_cafe_babe,
            0x0123_4567_89ab_cdef,
            0xf0f0_f0f0_f0f0_f0f0,
            0x1111_1111_1111_1111,
        ]);

        #[rustfmt::skip]
        assert_eq!(
            &num1 / &num2,
            Some(BigNumber::new(vec![
                0x25cb_7116_bc61_d3c5,
                0x6969_6969_6969_6d18,
                0xffff_ffff_ffff_ffbd,
                0x0000_0000_0000_0004,
            ]))
        );
    }

    #[test]
    fn test_div_nine_limbs_by_eight_limbs() {
        #[rustfmt::skip]
        let num1 = BigNumber::new(vec![
            0xffff_ffff_ffff_ffff,
            0x0123_4567_89ab_cdef,
            0xaaaa_aaaa_aaaa_aaaa,
            0x5555_5555_5555_5555,
            0x0f0f_0f0f_0f0f_0f0f,
            0xf0f0_f0f0_f0f0_f0f0,
            0x1111_1111_1111_1111,
            0x2222_2222_2222_2222,
            0x3333_3333_3333_3333,
        ]);

        #[rustfmt::skip]
        let num2 = BigNumber::new(vec![
            0x1234_5678_9abc_def0,
            0xfedc_ba98_7654_3210,
            0x0101_0101_0101_0101,
            0x1010_1010_1010_1010,
            0xaaaa_aaaa_aaaa_aaaa,
            0x5555_5555_5555_5555,
            0x0f0f_0f0f_0f0f_0f0f,
            0x1111_1111_1111_1111,
        ]);

        #[rustfmt::skip]
        assert_eq!(
            &num1 / &num2,
            Some(BigNumber::new(vec![
                0xffff_ffff_ffff_ffff,
                0x0000_0000_0000_0002,
            ]))
        );
    }

    #[test]
    fn test_div_hundred_limbs_by_seventy_eight_limbs() {
        #[rustfmt::skip]
        let num1 = BigNumber::new(vec![
            0xd2c9_22a6_db56_42ed, // lowest
            0x62bc_4d42_9190_6780,
            0xbd14_01c3_51ff_2306,
            0x5696_5b31_0337_4270,
            0xe959_5ffb_c462_a2ff,
            0x7243_9958_b5c9_af27,
            0x9254_ff55_8e1d_bae0,
            0xe3ff_d19c_525f_5804,
            0xadf7_4dce_ad71_e6b1,
            0x7d1d_9abf_d716_ae0c,
            0xd703_242f_79d1_be93,
            0x08aa_cae2_5b86_0314,
            0x7940_3e0b_faf8_735c,
            0xb535_ab18_53c5_3ead,
            0x051d_799c_0a40_931c,
            0x8c66_be19_6522_a490,
            0x8388_9015_c35c_fd14,
            0xaa12_5c89_0b3c_7619,
            0x970f_a6e2_ff1d_126e,
            0xe8f3_d977_58bd_3a6c,
            0x9cfa_f4c9_eb4b_7e61,
            0x45fe_f556_7527_6d02,
            0xc938_5d5c_cf1d_ec64,
            0x5b7e_24b7_faa8_a32b,
            0x0d53_c009_ad4d_a8b6,
            0xcf44_d5c9_8e55_5b17,
            0xbb09_126c_eb4d_cc48,
            0x0457_1bcb_dde2_57a2,
            0x301e_23df_59d8_2f08,
            0xd46b_92fe_f6f3_e379,
            0xfda2_1cd8_2ddb_2a16,
            0x0eaa_db90_36b7_e56b,
            0x08d0_1dbd_ef25_0437,
            0xeea6_e23b_354d_84b8,
            0x5b29_f053_9d32_1e28,
            0x7dd9_d0d5_5186_1db6,
            0xd42a_e9b1_523c_808a,
            0x9b73_8542_3419_68c5,
            0x5226_97f9_3283_d80e,
            0xd85f_d2f7_b0a6_a260,
            0xa5b3_2ab1_564c_7c39,
            0x516d_a835_f454_fb10,
            0x472a_e0a6_4dd2_2492,
            0x8a5d_be57_071a_c53e,
            0x3eea_bca8_8688_d9f1,
            0xb16d_7966_0d80_74cf,
            0x6238_2c3e_b218_1f54,
            0x655a_f396_9a9e_e77f,
            0x4064_c7fc_1110_e300,
            0xdb10_3eae_07c0_155e,
            0x60d2_46c9_9383_ba3a,
            0xb788_a718_c3b9_e2b7,
            0x713a_92e8_efdb_461b,
            0xd05a_2418_7402_55eb,
            0x42ff_bf8b_0ee8_3b5c,
            0x2daf_e20a_eeca_4c07,
            0x0a3d_0dc4_c761_de80,
            0x5e1c_a82a_fe4b_642f,
            0x3f38_0710_2c53_475f,
            0xa239_2295_64f1_19a9,
            0xfde7_3a38_98e8_e43c,
            0xe50f_e400_f207_4509,
            0xd1be_cc00_cc15_a3f3,
            0x0f20_a6d0_0662_81fa,
            0x7912_5615_d339_9a42,
            0x8fd0_fc95_2ec5_a18f,
            0x3b5e_037f_a3e0_a66c,
            0x7e5d_d977_773b_c572,
            0x7ef1_c8ff_4ac6_90ca,
            0xbe01_286c_97c3_fca6,
            0xbd6d_6ef0_85a2_6814,
            0x66b1_81f2_5ba6_7ec9,
            0xcb9a_1802_028b_53c5,
            0x5675_84c9_b7e9_6069,
            0x5f32_056b_8916_db66,
            0xa587_1d79_4725_1fb8,
            0xb1a5_f231_1253_d794,
            0x7b74_b050_179a_526f,
            0x4a69_c473_b8c6_7570,
            0x7617_1702_3ca8_475e,
            0xabba_2e2b_9e22_64a3,
            0xbf12_484d_22fd_125d,
            0x5ec9_0ca4_a573_78a1,
            0xba33_7602_6659_6c17,
            0x3c4a_8e03_0ffa_5fff,
            0x8b4d_a050_513a_2227,
            0x8c7f_89f3_c644_329e,
            0xbbae_e6c9_d199_be6b,
            0xada2_0117_1d34_ac6b,
            0xe6c8_53c3_76c2_ed0d,
            0xf44f_eb81_435a_0b1c,
            0xf384_aa57_9370_26a0,
            0x97c0_de36_fd6a_102c,
            0x1266_7977_3540_dfb8,
            0x0fec_1ee1_0408_a1a7,
            0xd2b6_dfe4_7aaf_b799,
            0xf60b_df32_1d47_696d,
            0x32df_ff85_f173_0f1c,
            0xc9fe_45c1_264e_96fd,
            0x7546_d97d_5cce_944f, // highest
        ]);

        #[rustfmt::skip]
        let num2 = BigNumber::new(vec![
            0x2bb7_4264_5f2f_e172, // lowest
            0x1373_6e13_89e1_77e6,
            0x0f96_3db5_a325_32ac,
            0x1a2d_5c30_e41e_43aa,
            0xc9a7_0cf7_baf8_6ce0,
            0x9861_d29e_c25f_c1c5,
            0x32d9_87c3_7fac_0211,
            0x021d_b876_1676_1925,
            0x16d4_6165_31ab_684e,
            0x4433_8ca7_b440_89a4,
            0x38ea_e41a_1d04_3fb0,
            0xd3ce_2868_3e4f_11ee,
            0xd5f3_f9ba_6b2d_7c9c,
            0x3d91_7d68_d70e_91c3,
            0x1ea9_125d_e8a9_b57e,
            0x15ff_5f79_1a57_5c53,
            0x09f8_6316_1061_7b51,
            0x5f3f_dd0a_4a97_c443,
            0x417c_c72a_8a90_f87e,
            0xf634_3618_f882_0469,
            0x298c_cd2c_d436_8361,
            0x9ce3_dc04_c388_95ab,
            0x9e33_effa_f3c9_d49a,
            0x0632_2067_01e8_09cb,
            0x37a4_9dd6_2423_b56f,
            0x2e23_cdb0_755e_ca77,
            0x71a1_00a3_6062_c37b,
            0x4394_1671_72f6_feb5,
            0x675d_f605_9f29_c908,
            0x6414_3534_313c_1cca,
            0x3d97_8c86_9d7a_3c29,
            0x2154_310b_d8fb_7a64,
            0x4708_44c8_dbb8_8915,
            0x4f27_60c3_b761_2ee7,
            0x404c_1f63_767f_713a,
            0x99c5_1087_ce38_63f9,
            0xca13_32dc_6a80_9722,
            0x4ae6_7d05_a7d6_49fc,
            0x598c_d6ea_c07f_a9b1,
            0xc486_9bf7_f360_7fdf,
            0x60c8_4edf_3af3_79dd,
            0x23d7_c3fb_5169_a7d7,
            0xbd55_2067_2ee2_be7c,
            0x1511_a3db_96b8_03e6,
            0x41a7_afe1_2af6_f197,
            0x8436_8a0c_041c_e756,
            0x1463_8942_adf1_41b0,
            0x5514_39d8_68fd_f065,
            0xa393_45d3_02ec_75be,
            0xdc75_1579_90c9_c261,
            0x0479_fa0e_9a01_cbbb,
            0xdb37_9657_e786_033d,
            0xcf36_b936_ef61_936d,
            0xaa62_1786_1080_ee87,
            0x05a2_4586_5ec1_1041,
            0x7108_4b28_ffe7_e685,
            0xe2b0_bd23_24a0_7e0c,
            0x3952_87d3_8958_6a2b,
            0x7754_7952_35dd_6850,
            0xa8a2_6901_dc64_32b9,
            0x2577_6ba5_dadc_9659,
            0x24a4_4cf9_df8f_9ff0,
            0x7436_fdcb_24c9_f91d,
            0x18d2_f068_3164_773c,
            0x8f55_e232_3d11_d42e,
            0xafa7_397d_a65a_d506,
            0x1817_f69a_f323_ac46,
            0xd840_1e1a_3edc_0f3a,
            0xea69_9b1a_675f_4415,
            0x4dbe_9942_1f1e_2649,
            0x6148_8543_9444_1e41,
            0xe65c_738f_9f97_2d56,
            0x2626_6568_31a1_7971,
            0x9506_9ce9_1350_4afb,
            0x9297_a315_448f_4fee,
            0x7ac2_0119_51f6_6953,
            0x6593_969b_09e5_99f5,
            0x37f7_c102_1649_eb3b, // highest
        ]);

        #[rustfmt::skip]
        assert_eq!(
            &num1 / &num2,
            Some(BigNumber::new(vec![
                0x01e8_827f_0087_2e06, // lowest
                0xd09b_ab1c_bd36_f295,
                0x7211_f6a4_1fee_2c00,
                0x1756_df13_8653_a3ee,
                0xdf9d_ddde_5b71_953b,
                0x8b71_c15e_0287_eeb1,
                0x890e_f279_6f4a_a976,
                0x0d68_0b5d_8d9e_0eaf,
                0x1e60_d328_2b62_4a5c,
                0x979f_1ad7_2e13_6e7a,
                0x750d_539d_2f81_8a99,
                0x1624_b960_9a30_12e4,
                0xdba7_e743_ac20_f4e7,
                0x9bb2_c8b2_39eb_cad4,
                0x4755_999c_1f41_2f76,
                0x77c1_12c9_fbf9_2f77,
                0xeeb9_ea7a_b00a_c061,
                0x0c82_da95_aded_7709,
                0xe6b7_d35c_ec8f_9621,
                0x9e42_9bcf_40a7_0fd4,
                0xb648_0b30_a401_ad47,
                0x186e_4d67_1931_3350,
                0x0000_0000_0000_0002, // highest
            ]))
        );
    }

    #[test]
    fn test_div_thousand_limbs_by_three_hundred_forty_five_limbs() {
        #[rustfmt::skip]
        let num1 = BigNumber::new(vec![
            0xe65d_46ac_4f42_c247, // lowest
            0xefc7_91b0_3508_7360,
            0x6bd7_c07a_3867_4389,
            0xbe58_c219_7caf_d6cb,
            0xa4a7_6c17_c7a0_7622,
            0x79bb_5539_a19e_eb2f,
            0xa04f_f288_77be_bb45,
            0xacd3_3830_ce1d_0ac6,
            0x6287_3747_3432_ee58,
            0x7f2c_3436_9f94_f5b1,
            0x8ae4_f4da_ccd6_fea9,
            0xcea8_efd7_1009_6dcc,
            0x5686_f871_6237_7773,
            0xc4e4_7b0d_3993_fc29,
            0xd89a_193c_5217_3e1b,
            0x4aa5_a628_f521_ce37,
            0xcdaf_bdb3_73e9_1bfd,
            0x1ae7_1350_039d_6e39,
            0xfdf3_fc38_f998_f416,
            0xf1a2_3748_0f41_6556,
            0x0ea7_8cd1_da8e_42da,
            0x285f_f885_14ca_febf,
            0x81f2_a3fc_e0bf_daf5,
            0x86b4_2292_72fe_70f4,
            0x1344_a2d2_985b_415c,
            0x1c87_d92e_60ad_ba05,
            0xdd15_b303_44fb_9e6e,
            0x8439_a27d_e639_4f65,
            0x12a8_aa01_c2ab_6f35,
            0x88c0_a141_63df_9236,
            0xc495_589d_e430_1667,
            0xb5dd_87f6_87d2_a46c,
            0xae22_4607_406d_72e6,
            0xdf14_1a7e_3be4_1202,
            0xeba7_5f9a_c6ca_5eab,
            0x81eb_1dc0_b270_b88b,
            0x693e_c0e9_8105_c532,
            0xd809_e9cc_cc47_e63f,
            0x8937_865c_4645_d245,
            0x5719_7d90_d54f_ab8f,
            0x45ed_f424_b061_cc7f,
            0xd8bd_8200_8ea8_c0ba,
            0xa889_c71f_5a69_a170,
            0x274a_ed95_e0ee_8c25,
            0xe384_d5cd_192b_eb17,
            0xb50a_d4a0_219f_0f18,
            0xf616_7454_cc6d_7c94,
            0x79eb_b4e6_b771_9baa,
            0x38f1_9b39_dbed_660d,
            0x0323_d5d6_45bf_8007,
            0x5e30_eb60_f623_2ec8,
            0xe4ee_3100_b2f2_0383,
            0xd9c7_4ed9_929d_2f08,
            0xe835_7fae_011e_036a,
            0xb088_42db_01fa_0178,
            0x717e_8657_c9ba_8492,
            0xca11_9fcd_00e2_7fc0,
            0x9bd3_72bb_253f_b864,
            0x2157_a286_0bbf_b3ff,
            0x59eb_5d7a_f6f4_33c2,
            0x26a1_9cb7_a11a_1765,
            0x6c0c_6808_ea17_2535,
            0xbda8_8cd3_4047_455a,
            0xabc4_0947_0181_0948,
            0x0fc0_0194_0d13_b57e,
            0xd2be_695a_7ede_c204,
            0x6b9a_4a72_5417_f849,
            0xe375_197b_d4e5_f3ae,
            0xb215_2788_bc09_6e64,
            0x2e08_64d1_be97_99e7,
            0x02ae_8dec_32d2_28bf,
            0x26f6_4fe7_3313_420b,
            0xdac9_db5a_9395_b802,
            0x510c_75a2_4dc0_1f24,
            0x89c8_b3f4_89b3_909a,
            0x6060_7a79_52ae_50f8,
            0x2fa9_2326_b80a_7bd3,
            0x0a16_9141_1caf_3746,
            0x2f93_a122_c8a2_e065,
            0x85c5_b4fc_686f_8bfd,
            0x17a8_2dbe_6beb_ee3a,
            0x8a34_988a_0040_ac12,
            0xc334_8144_6224_588e,
            0x9da3_190a_08d6_4f2e,
            0x1306_0f48_be25_e4a1,
            0x2ebb_8f96_a51b_d8d1,
            0xbd12_589a_4cfe_72ce,
            0x8e2b_5402_591c_29aa,
            0x3618_945a_ce06_b252,
            0xeaeb_ecfd_5cae_7dc3,
            0x7954_f81f_9a07_fb24,
            0xeb5a_3510_3baf_ba0d,
            0xda42_1dc8_70b0_8fac,
            0x4083_d9ec_f946_3fc1,
            0x5fbc_418a_928b_6d9a,
            0xe2b7_247a_c355_5ba6,
            0xb8f2_0c1c_1a5d_2bde,
            0x8ba1_c8db_9722_a21a,
            0xbbec_bf48_bc24_bae3,
            0xe1d0_0883_ab12_c701,
            0x288c_c4e5_c6f3_7162,
            0xf2b0_5947_df13_606b,
            0x859a_9227_8a66_55ee,
            0x6355_9d73_60a5_20b7,
            0x4c65_c44e_a186_a3ad,
            0x2175_99aa_47e9_1b04,
            0x4e76_6d2d_edd6_3b23,
            0x67ab_335f_67a3_807d,
            0xc08f_df70_646c_91c9,
            0x8cb1_10d3_a24a_830f,
            0x8aa5_9693_9716_eb5b,
            0x6589_4d9f_db90_b3d5,
            0x32dd_bb24_21d3_95f7,
            0x4ab5_a369_ab12_730c,
            0xccfb_9823_bca3_b4f7,
            0x6bb6_c5f2_e1eb_c538,
            0x2a6b_2152_8887_53b1,
            0x48c3_f41d_3029_1432,
            0xf843_b4d0_b602_425d,
            0x96b7_397b_3ec4_27ad,
            0x81a2_4d94_b91c_37e9,
            0x79f4_f2e0_470d_3189,
            0x2f7b_327c_46aa_0484,
            0xb088_06f4_d143_1feb,
            0x2731_6b02_e3c1_1fbe,
            0x3f88_bcdc_c3fd_33be,
            0x0bee_14c7_9e1e_d06b,
            0xb085_2db7_b82e_6112,
            0xa831_a2af_7540_0557,
            0x3c0a_acc7_3b64_ec7a,
            0xd094_041e_2a95_fbf3,
            0x6a12_dd74_646c_4736,
            0x50ae_05e9_7982_0acf,
            0x73f2_f5e9_c5f8_5ab0,
            0xc7de_3e0f_060f_3a93,
            0x7678_2f24_0a70_146b,
            0x3932_b8e1_83f8_6993,
            0x6db0_5b86_a07a_258a,
            0x68a1_70aa_447f_d4f9,
            0xe846_ab7a_b7f6_86db,
            0x1406_e71a_9b2c_3256,
            0x90e9_3a27_c490_bd49,
            0x8a7d_2f95_ca62_0cf5,
            0x37e4_73ed_bb55_2948,
            0x4162_6df5_929e_964b,
            0x972a_493b_d65d_70ca,
            0xfcff_d219_997b_8cad,
            0xce26_83a2_f308_2ecd,
            0x2158_2195_5dba_6907,
            0x852b_2ab7_df75_54d6,
            0x0aea_6921_3884_e5e3,
            0x8828_6c4d_0f3f_dee5,
            0x2c8b_92c3_ce6b_4e86,
            0x03a8_fa21_ba3b_ff89,
            0x6893_5db2_76d7_0bf9,
            0x7a45_ea6c_d66a_96b6,
            0xb31e_9a90_55c8_df18,
            0x58dc_f879_1efd_aa91,
            0x7d35_7978_6106_f96f,
            0x7649_eba1_e346_0cf7,
            0xe917_dbe0_c321_e729,
            0x4157_8696_d642_43bb,
            0x221a_c5c5_52d0_9f0e,
            0x1d3b_5cd9_316e_1bca,
            0x177b_889b_b9ea_c5c1,
            0x41c2_513c_d6b4_a9e3,
            0x3562_bf98_4aec_6eb1,
            0x311a_54e6_7048_6847,
            0x6ce5_1ed5_4052_c12b,
            0xc621_c751_3c67_b1b0,
            0xbaf6_7a39_6593_be09,
            0xb7fd_6eee_879d_e1ac,
            0x767d_dfe9_accd_72db,
            0xdbec_476b_1645_cd34,
            0x4c4b_5cd3_2efe_1cd9,
            0x595f_7208_8366_ce92,
            0x22a7_7b9e_28d9_c672,
            0xecd3_a767_112b_aa28,
            0x1a99_9f63_9f30_05ae,
            0x4241_ffba_f1d3_01b0,
            0x7b6b_258b_3629_c11b,
            0x311a_2c94_4e75_9744,
            0x3247_47f2_167e_646c,
            0x530d_5bef_1ff1_1ac1,
            0xae83_2e51_eb89_8151,
            0x6299_be95_8b88_d497,
            0xd028_9e39_f8c0_ec1a,
            0xa500_8ff6_8d65_05b3,
            0x00c8_3424_6209_e9c9,
            0x8d9b_156c_84e9_9601,
            0xe9e3_30ed_f660_4f79,
            0x1db2_3d2a_7f46_f1f9,
            0xd213_174e_b0dc_71ad,
            0xc05a_721a_c195_7024,
            0x2bf7_0575_d7d8_a926,
            0x1560_933a_d517_0de8,
            0xc314_342a_90ff_6c97,
            0xb0da_7939_6002_6c00,
            0x85ed_4982_1f8a_39d3,
            0x0f45_5fba_00f1_f6c8,
            0x8fd0_5f1c_b0ee_3e0e,
            0xd088_0ee0_913a_19d7,
            0x9d37_0ee0_ba52_a6bf,
            0x6ee5_a7d4_b594_693b,
            0x8b4b_7c42_1aaf_778b,
            0x5725_316e_0fa6_657d,
            0xae7b_c0a8_6dff_ac51,
            0x9860_9729_26c8_cc9e,
            0xf196_4f1a_1ba3_bd74,
            0xafcb_0f48_10f1_ce92,
            0x6951_9401_2f04_954e,
            0x4d4a_98d6_62d1_e244,
            0xbd01_c7a6_9c38_7f5d,
            0x46ad_6042_570e_0f71,
            0x4209_9cc9_4186_5d58,
            0xf81f_8dbd_0bcc_1ae8,
            0xac84_3d3b_85fe_9ad1,
            0xa145_388f_9f4f_88a8,
            0xc6d9_dad6_8563_3e96,
            0x3426_d9b1_8160_6406,
            0x7a8f_43e8_6f16_25be,
            0x38e9_6b92_5e44_0af9,
            0xb246_a9b5_a457_30b2,
            0x15bd_9750_9668_1a4a,
            0x5e79_cba9_55e8_ef99,
            0x93bb_e2c3_b3c4_2326,
            0x786b_657c_8286_e84a,
            0xe110_3c10_22e8_7f2c,
            0x1678_c790_bbc6_8465,
            0x3614_03fc_2f70_faac,
            0x24c1_537e_1ff4_d419,
            0x5ff7_10d9_d458_c1f0,
            0x03aa_8e32_bb7d_81e6,
            0xf115_1d61_6905_fe33,
            0xc1fb_1d81_1ba8_ec38,
            0x372f_445f_fe35_1b25,
            0x558b_6967_3757_6ba0,
            0xcf7f_1449_3700_1914,
            0x4861_3ea2_8070_329c,
            0xbfce_4728_f3ff_06ea,
            0x680b_e24f_0c80_a2f5,
            0x9e9d_5873_eb97_41cf,
            0x653b_697c_8ec5_f1ce,
            0xc261_374a_41c3_b774,
            0x77ce_baff_62bd_9246,
            0x4cbd_4b9c_f4c0_e739,
            0xb06a_bc7f_c193_3530,
            0x5613_516e_ce39_84cd,
            0xe25e_adbd_fc55_3360,
            0xa730_1884_6b07_a370,
            0x88c1_e07b_cef3_6687,
            0xb5fa_ffd3_2ce9_6149,
            0x99ff_38ab_790b_34a9,
            0xe404_f361_86e7_383b,
            0x013c_72dc_c550_a8ba,
            0xa83f_0f11_74a9_88cf,
            0xd0da_afeb_44d2_2ac2,
            0x403b_80b9_fc29_f88e,
            0xb073_77a9_ec39_eec4,
            0x2519_6971_2529_ed52,
            0xe2ab_ca61_0b30_bbc2,
            0x2292_c411_912a_1726,
            0x6b28_00f2_f1bc_4367,
            0x6fcb_f6cb_31ae_4b61,
            0x88e4_ca57_3d29_de37,
            0xcc2b_72f2_04d4_7d0c,
            0x6ef5_f8cf_2a3a_6090,
            0xaded_3827_1213_59ab,
            0xfa58_da9a_f8fa_1c94,
            0x0d17_1d4a_d7fd_cce4,
            0x678b_fa6e_3c66_1ca1,
            0x6fc2_73be_2fbe_ed16,
            0x582d_51c0_ede3_6471,
            0x700e_ca09_8c5e_264c,
            0xa3b1_8677_bc42_fccd,
            0xde93_04fd_5747_b0b6,
            0xac8d_fc75_a49f_2d59,
            0x54f8_6586_0acc_1d7f,
            0x7fe1_5577_d28c_49f5,
            0x17cb_8685_b563_e2cb,
            0xd883_bca8_9326_7d95,
            0xaf16_531b_763d_f3d3,
            0x43e3_234d_a242_75f6,
            0x62f7_81b8_ed1b_bc35,
            0x16bf_1010_421c_1470,
            0xc039_4425_afd2_c63b,
            0x903e_7383_3447_6781,
            0x9997_845e_9edf_802c,
            0x0842_e18c_ac8a_c0a7,
            0x9e2c_332c_7a79_5c40,
            0xbcbc_77e2_ca35_9b48,
            0x3591_989a_19d4_6feb,
            0x0623_55d9_3902_1261,
            0xc9fe_cbd2_6282_7242,
            0xdec2_f97c_c32e_9036,
            0xc5c9_21ae_e8a1_f3d3,
            0xe763_192e_3806_ff4b,
            0xd991_0e0d_8df8_0169,
            0xcd3a_e97a_cfa1_e456,
            0xef7c_c8b7_c109_6dbc,
            0xf08c_b98a_43cb_fc33,
            0x9615_7317_7115_9f7d,
            0x0c95_d846_b585_4865,
            0x26f1_f728_a94a_ad68,
            0x3617_ac79_d343_e389,
            0x04d7_b0f0_6679_0cb2,
            0x57fa_9bc0_712f_145f,
            0x962d_eb3d_91e8_c525,
            0x5e27_807b_b172_b5ab,
            0x9e1f_bfa7_40c7_eee8,
            0xd29a_5646_ead8_d74e,
            0x4391_af88_9684_138b,
            0x1497_4315_288e_6711,
            0x2046_d7b1_821e_d703,
            0xbb73_0d67_4001_25fe,
            0xc931_01ce_f529_8b7e,
            0x7096_be2e_426f_5741,
            0xfeff_b811_2f2d_7953,
            0x317b_08f3_f476_3ab4,
            0xceb9_050a_56af_c36b,
            0x1988_47c6_c4f0_ae95,
            0x1f7c_45a9_3932_1e6b,
            0xca72_4153_88a3_a354,
            0x532c_eb05_0f54_b050,
            0x60db_1e18_c893_494d,
            0x5523_52b6_201e_90a2,
            0x6be7_f91e_b575_68fe,
            0x72aa_14e7_0fd6_8d12,
            0xb4c2_0621_877e_ddd9,
            0x4625_3ebb_0da3_2762,
            0xfe3f_9d5f_f566_a9a3,
            0x4a71_1efd_631e_e8c8,
            0xa68f_b5d7_d14f_d285,
            0x4233_cadb_521c_e36e,
            0x802b_7145_8a7e_61c5,
            0x5bf8_c14b_585e_5426,
            0xf7df_9cb7_7952_7343,
            0x1c0c_cf52_9ef0_991c,
            0x82ab_a1d3_32a1_b37b,
            0x0ddc_95ef_8e4d_6236,
            0x7c6b_c02c_b233_e56d,
            0x6beb_5a75_be8e_c7e8,
            0x4d0c_0e44_069d_dfe7,
            0xdb88_f534_1eb4_3eca,
            0x66a0_0d8b_fcee_d36c,
            0x9513_cca3_d203_e1b2,
            0x2aec_8bb1_cc19_e3b7,
            0xada1_06f6_8f15_71f3,
            0x0cfd_9ad5_cf72_c5c0,
            0x726a_a507_d54a_3bf5,
            0xb1e6_1167_8a32_daf8,
            0xcc61_bf8c_835c_cb7c,
            0x606a_bc88_968e_7490,
            0x88d7_43db_b1fd_28f8,
            0xcf5f_bed8_f5b3_81da,
            0x751a_d598_90e0_58ec,
            0x7b47_0952_64f1_3f82,
            0x42c0_93c8_c81d_05a8,
            0xdd1c_29f3_ee46_b723,
            0x8086_dbc4_b145_4259,
            0x82fb_329b_3b52_ca95,
            0x8ea1_8aa1_bea3_aeca,
            0xdca7_01d7_38d3_70c6,
            0x1a40_9020_4028_507d,
            0xb148_7b27_29f1_02ed,
            0x7b0a_c18d_32e5_6d38,
            0xbfd5_9dda_f5cf_f788,
            0x9603_e48c_1ee3_d4c5,
            0xc507_1ac1_9249_31e1,
            0x7377_44dd_0d24_ad24,
            0x1419_5818_f396_c245,
            0xcdfa_619e_e98b_30a8,
            0x48eb_42a3_251a_3588,
            0x24ef_6ea9_8011_9ff1,
            0x5769_b36f_4b66_1b6d,
            0xfe2a_7861_58dc_5f16,
            0x1a06_584a_0960_542a,
            0x7051_0b5f_eeda_3543,
            0x6b40_bd07_8b83_7831,
            0x83c4_a105_e645_5c58,
            0xf68d_a273_ebd3_7db7,
            0x95c8_dd9b_c22f_9d57,
            0x5639_640d_64a0_4b2d,
            0x02a4_b74e_6afb_4af5,
            0x2cbf_7094_d022_4dc8,
            0x2861_a9d2_f5ad_44b9,
            0xb17f_841f_6f79_c040,
            0x7d27_269a_f8bb_0899,
            0x2ed9_2eb2_12d6_b8a5,
            0x91f4_5c3d_962c_d38d,
            0xd878_dada_64c8_e44b,
            0x4a3a_96a3_98f5_6160,
            0x558b_8331_eef6_4db4,
            0x3a06_ec64_4213_3b87,
            0x1740_d067_8d38_d126,
            0xab19_ddd6_a0ca_3976,
            0x19bd_f09b_95a5_7e9d,
            0xa07d_abda_14b0_b7bf,
            0xfe0f_8fd5_e90d_7c0c,
            0xd2b7_fa08_47f1_3812,
            0xbce4_b352_0ffe_771c,
            0x2638_4bb0_d3de_8e21,
            0xc85b_2487_3339_6bc4,
            0x6bb9_7982_7acf_7e46,
            0xe97a_a5e6_f302_3dfc,
            0xb37a_dc03_d50c_8dbb,
            0xadb2_0807_769a_0256,
            0xc99b_8235_bb64_081a,
            0x8709_ca15_daf3_84fb,
            0x3392_5a8d_e1de_fc91,
            0x199a_a45f_5e6f_9c3f,
            0x9575_ae94_07d0_f3dc,
            0xa504_387d_c128_a60e,
            0x5207_3de5_d99a_2719,
            0xb337_e86d_c600_ad70,
            0x6875_1b5f_0756_aa55,
            0x1d09_d2e5_37ff_6b6d,
            0xe5ad_5f53_17cc_8f6c,
            0x4bfb_e467_2b34_b1ac,
            0x4379_45b3_6c32_d858,
            0xc352_a41f_da93_bd03,
            0x2df9_8864_190a_9c75,
            0xd400_4c31_ae1b_0ee2,
            0x499c_6757_c16f_7551,
            0x45d8_0d88_d320_d1ec,
            0xe66b_64eb_98af_c1e2,
            0x3841_6b73_01ca_2901,
            0x2b6c_03ae_1788_11a9,
            0x06eb_5a6b_fc43_318a,
            0x31c4_03c5_7227_089d,
            0xeb19_b468_7ccf_cf67,
            0x697d_9795_3a0e_debd,
            0x1d51_f4ae_9bb4_116f,
            0xb51d_0450_a14d_2ebc,
            0xcf86_b695_c8d3_8cca,
            0x109d_a06c_1c0f_0642,
            0xd16a_dc87_21b9_a153,
            0x29ff_567c_9181_9b9d,
            0xa647_e282_99c2_6918,
            0x5efa_1154_8755_2c75,
            0x7d16_1643_7ae8_b2a0,
            0x18cb_4de0_cd7e_a347,
            0x698c_c630_1b9c_622a,
            0xf7e5_feff_33d6_c825,
            0x47a9_1d44_3287_f96e,
            0x8b47_60ab_8b6c_e5b9,
            0xae3f_5bcd_cc17_6da5,
            0x0b65_878c_7dd9_bdf6,
            0x85cc_b8c6_4051_54c1,
            0x623c_c14a_1bd4_21e8,
            0xd7ae_334d_f993_074b,
            0x23ae_420e_825c_d566,
            0x9d66_1d93_1392_d91b,
            0x633e_3967_b58d_de7d,
            0x9e88_245b_649c_b47f,
            0xedac_37bc_907e_b638,
            0x43b8_7d88_aa2a_8d02,
            0x70db_6723_8ee2_7964,
            0xff3e_ef52_c657_f68a,
            0xa881_3bd3_a0e5_f689,
            0x8d8e_aa9c_047f_63a0,
            0x0c74_7bc5_6f4a_f2b4,
            0x803a_d165_bf3b_86e7,
            0x9aa7_a3da_e0f6_20bf,
            0x475c_bad2_0218_52b8,
            0x8279_d605_6a7b_c93f,
            0xa7bd_cbb8_19d4_2d04,
            0xa5e9_4cb2_5466_6b1b,
            0x7093_f0df_0e81_7bc7,
            0x3b32_798e_9320_e9c4,
            0xe504_c758_52ce_41af,
            0x0a42_85e7_0a13_d3b2,
            0x9b8c_c18f_5449_8f6d,
            0xdd20_7d54_7c24_748e,
            0x7059_6a2d_3516_343f,
            0xfdd4_8844_8699_607d,
            0xdfa4_de97_2ba5_7d6e,
            0x7977_6a12_6d3b_5b2a,
            0x0baa_ae84_fdb9_dd28,
            0xb5ce_d0eb_1593_e3af,
            0x486e_26e4_242f_f53a,
            0xb52f_897d_3560_360f,
            0x035a_80c4_7e3a_88fa,
            0xcd98_1a97_a7d5_c0f2,
            0x599e_413d_b5db_beef,
            0x5635_e2be_feb5_62cf,
            0xe438_bd06_8c43_5bcb,
            0x5b01_eb51_39f1_f755,
            0xc3db_d930_55cc_6bb7,
            0xacd7_af79_6f40_d8fe,
            0x71fc_999f_6f2d_13d1,
            0x315e_5a33_5a73_ed2c,
            0xf306_7cce_36cd_ba0e,
            0xd76a_114d_97a9_30da,
            0x8aad_9125_f299_1e88,
            0x2296_9697_01a0_e5c3,
            0x6276_7c6f_ad54_b4d7,
            0x0f2f_5390_fd56_8875,
            0xa6c4_d801_c5d7_bf8c,
            0x7e37_ca4a_6a23_ed7e,
            0x1ed9_6bc4_38fa_5990,
            0xe0a1_1d10_56c4_8e86,
            0x2754_a08a_038c_55be,
            0x6244_3ddb_d3eb_2d57,
            0xaad0_bf54_5653_455c,
            0x2e24_d22a_882d_77dd,
            0x33f8_5550_e006_f838,
            0x6f74_cdd7_3c5a_2255,
            0x9533_83a9_a481_4cfd,
            0x27a7_a9cc_eb03_02ef,
            0xde9c_7e1c_8886_ff9b,
            0x6367_6310_49ee_b101,
            0x9800_1d9e_aba9_8b1f,
            0x143e_898d_f8ce_a28b,
            0x66ad_c766_a4e3_07d0,
            0x2412_b4f3_e85b_9d47,
            0x13b8_e250_2d0a_ea1e,
            0x8682_45d1_b4a6_0450,
            0xc2ca_7538_0138_3ba3,
            0xecb9_ab9b_26fc_d525,
            0xb6a0_3827_174c_100d,
            0xaf9c_ef8a_121c_2ac9,
            0x52c5_0ee3_3dd0_bf6e,
            0x4bba_acbf_f20c_f789,
            0xeefe_0e0c_56be_79a4,
            0x6c10_dca1_f793_9953,
            0x03fa_b0ea_48b6_7977,
            0xbead_4ea0_8464_a273,
            0x5c86_4985_6ec5_a2c7,
            0xa2e0_0751_9752_2a5c,
            0xfde0_e70d_d7d8_64eb,
            0x1f22_567d_1a14_aa68,
            0xdbbd_7a79_42ce_983d,
            0x24f8_54de_7c6e_f0be,
            0x3f56_5715_e806_6705,
            0xdb75_8e20_7f84_771c,
            0x6c98_ead1_b101_158f,
            0xda08_94f7_e0f6_0fab,
            0x0594_d0b5_9768_987d,
            0xd138_aa83_7d54_2066,
            0x349c_4f60_012b_8b3e,
            0x2e0c_c7cb_f691_0f28,
            0x75f3_de9f_8661_8f65,
            0xbe08_8a36_5f8b_2f54,
            0xad94_efe6_3d52_bdeb,
            0xc0b5_d6da_208a_0d19,
            0x2043_e8a5_5eab_c28a,
            0xd8a9_0677_f9d4_04e0,
            0x536b_16e8_b097_1352,
            0x1f7a_86bb_e9ae_4809,
            0x9289_e633_6a01_e21f,
            0x0814_93e4_991f_6850,
            0xc1c2_a776_d89c_8f5f,
            0xd5b0_858e_5bbd_6c88,
            0x1255_68cd_b735_7e9d,
            0x55fb_650a_92ce_f041,
            0x5848_e88a_7de0_2f0c,
            0x9eaf_6524_d5b1_4f74,
            0x61c6_a702_c85b_b7fb,
            0x241f_75b4_3c1c_009a,
            0x9568_6c7c_c93a_9fc5,
            0xd5bf_ae9b_2925_0c30,
            0xaa1a_fa19_07ff_f3a9,
            0x7997_063d_2de0_afe2,
            0x411a_1d1e_9bc8_04c9,
            0x5083_7bde_9dca_b832,
            0xbffa_88f3_a6b4_d1a0,
            0x7563_bea7_532b_8216,
            0x5bac_01d9_a7d3_1e6a,
            0x256e_2a97_3e38_2b7b,
            0xd2f5_8615_58c5_8287,
            0xd3f2_e8f4_7f71_3739,
            0x908b_c642_dce1_c1a3,
            0x9cb1_1ac9_2f3f_876f,
            0xe345_2a4d_f77b_cb90,
            0x5b77_ae26_9cbb_5f4e,
            0x95fb_c1df_984b_a55d,
            0x8f12_dc48_d5c4_bdfe,
            0x3d12_894f_b740_6d6b,
            0x443f_7708_3ade_93f5,
            0x4849_3071_0d21_3c98,
            0x2887_9eb8_13bc_7539,
            0x8691_564c_f819_e91e,
            0x4052_f304_e334_188a,
            0x5b65_723a_628d_6110,
            0x8d00_9166_5b2a_04cf,
            0xdee9_fcbe_6cfb_e925,
            0xd483_c54b_1b6a_a8ec,
            0x47ca_35c9_b6aa_c270,
            0x8e40_3f42_f1b2_559c,
            0xe444_7f8f_0276_4136,
            0xc872_17a4_8d5d_6462,
            0x5824_8837_f8a4_4139,
            0x8a45_81e8_5999_9b78,
            0x5dc2_7506_58e8_4fe0,
            0xb591_661b_1d8b_d004,
            0xde70_529a_fdbb_d36a,
            0xa164_7a91_e11c_2839,
            0xac62_cbb4_f1f1_0deb,
            0x71e9_3a40_d0c0_6c30,
            0x822b_1559_83a3_5e29,
            0xa34b_cf50_a98f_eb30,
            0x88ee_8ee6_5514_b73c,
            0x2c71_83f0_0c2a_471d,
            0x2a47_7075_66fb_2c49,
            0x2113_e4e9_754d_cf12,
            0x3e54_5111_7bb8_e2ec,
            0x84a3_7f7b_775c_5afc,
            0x0beb_ef45_e4c5_a709,
            0x2cfc_7d83_3581_a0f6,
            0x330f_ddd8_e2e2_f23e,
            0x492e_1393_5e06_d657,
            0x4dc9_090e_26fd_24f0,
            0x654c_72b2_6a2f_3348,
            0xa9e0_9968_7a73_0b86,
            0x1e21_10cf_f102_31e3,
            0x097f_ae61_0597_fb8d,
            0xe52b_47ce_b721_2cac,
            0x5970_2fd2_730a_90a4,
            0x3b06_ce2d_7afe_ad40,
            0x8187_3586_602d_132e,
            0x63f0_1b63_323a_6fa0,
            0x1139_4917_b8d6_f38e,
            0x654b_be02_b7e7_1051,
            0x92ee_9fc4_39b2_ac75,
            0x271f_aa36_c941_264a,
            0xd37c_5f47_1c52_1735,
            0xf6d2_68e5_9a35_f1d4,
            0x5e63_b70c_a8aa_4735,
            0xc762_902e_afac_4e0f,
            0x7d68_b48b_04c8_19a8,
            0xb977_0181_c1aa_7433,
            0x92bf_f826_817c_fca3,
            0xf65d_a40c_d1e7_6c1e,
            0xa316_1620_a185_939c,
            0xd36a_82f9_4954_5f6d,
            0x63eb_b24e_7848_0d4e,
            0x02c3_38e0_7757_3547,
            0x73ed_8e9e_c064_3c94,
            0x45d2_9c7b_d693_ee12,
            0xece8_3653_e80e_125e,
            0x3ed5_e4bf_866f_96bc,
            0xedd4_6342_bc36_2249,
            0x5461_da20_3a01_309e,
            0xd5e4_87eb_ee5c_f89e,
            0x0e36_fc20_ef6e_dc74,
            0x5a11_4649_a32f_665f,
            0xdc6c_a491_14f1_ab67,
            0x289b_8f66_6509_b5c3,
            0xee0e_f47f_74e2_4e15,
            0x7eb1_ecdd_da9d_2a24,
            0x0f9d_76a1_fd42_d2f3,
            0xf8dd_7cc5_2f15_0f1f,
            0x94b9_90f7_44a3_5e59,
            0xca52_f818_2e59_5d5a,
            0x640c_8c7d_99e5_9a62,
            0x6afa_5d3e_ff0b_0f94,
            0xef27_0679_ec7f_9ef1,
            0xeb2c_5f53_f768_ce07,
            0x4470_f21d_2a59_ea9d,
            0x0d03_0327_2235_783b,
            0xbc7b_3744_a5ba_4e42,
            0x5b7c_9b76_cfd5_bd25,
            0xbc69_4144_553c_a7e6,
            0x82cd_da45_6d37_074f,
            0x8060_619c_23fa_cfbb,
            0x137e_e50b_14d0_47eb,
            0xe658_bcfb_6402_b3fd,
            0x8e3e_9111_2a08_b6aa,
            0x6082_0d2a_daec_873a,
            0x4444_6f94_4bfe_2347,
            0x4876_35e3_3c19_f461,
            0x7cb3_de91_0ec6_5d45,
            0xc52d_de11_6730_4aa3,
            0xb89f_d593_f1a8_7e16,
            0x5eb1_e6b4_52f2_4a1a,
            0x2a9e_9bc3_f1d3_24a6,
            0x33f4_9b96_87ef_72b2,
            0x53e8_095d_936d_ba26,
            0x56b3_50bc_7903_2823,
            0x2399_c132_f0b2_9886,
            0xd42b_8d0f_8362_57e3,
            0x460d_7c69_98da_16de,
            0x357c_4557_b86a_92d4,
            0x9e2e_fda6_42d7_2034,
            0xa98f_cb3c_164e_7578,
            0x3d18_0483_45d6_0ed4,
            0x8bfa_5833_42df_3813,
            0xea1e_2c5c_c1f1_c64a,
            0x0203_fe60_c733_73c5,
            0xfacd_66bb_d94c_dd8c,
            0xa7fe_d1d4_14c4_c9b0,
            0x55ad_f774_aec0_11a5,
            0x7dc6_dd83_f371_8d52,
            0x5dc8_98dd_aefe_2055,
            0x0da2_a46f_2360_a5dc,
            0x145d_530c_0c9b_184a,
            0x9434_d76d_4953_47a8,
            0x1a7f_d40b_1806_f44b,
            0x49cb_8266_14a0_2cfe,
            0xa4a9_435b_3498_66b8,
            0x1896_36e7_0998_a68c,
            0x6dac_d48a_da5b_99cb,
            0x31a6_998d_b3dd_eb8c,
            0x907e_9c92_e70a_b9bd,
            0x81ae_ab23_11d7_3431,
            0xd0a2_4446_6d36_33dd,
            0xd7f0_bb71_c672_6c25,
            0x8f40_34c6_fd84_8414,
            0xcfb6_37db_a23d_72ae,
            0x2cc9_0b28_6a91_8cd1,
            0x599d_81c3_2155_8df2,
            0x63a3_f981_a4fa_cbd9,
            0xa5dd_2fa1_6628_2616,
            0x0489_4ad3_9532_01ea,
            0xaa2c_d0e0_b884_d02a,
            0x58a3_fa53_3d03_dadd,
            0xf0df_e22d_af34_2693,
            0x650f_ffb0_db80_bf6f,
            0x1530_1a50_167c_97e6,
            0x0f44_fdfe_5c1f_cccc,
            0xd6e6_d552_c71c_673a,
            0xd1eb_bde3_0b18_3b2f,
            0x0131_6907_f0f9_e4e4,
            0x2506_44c7_914b_9994,
            0xd868_a35e_c7f4_6ca2,
            0x63d9_efe0_32d7_aae7,
            0x6066_b70e_5420_d8be,
            0xf7d7_bc7b_dd43_8dbc,
            0xbd5b_4f55_ead8_83e8,
            0x235b_0316_dc48_7e6a,
            0x5cee_84d0_0d6e_a085,
            0xb1c5_d0e8_20ca_b3d4,
            0xc6e0_10d3_ba1e_bb31,
            0x6a0e_6bd0_1270_6008,
            0x8777_c738_8959_0dac,
            0x3dd5_5b3c_28dc_8e70,
            0x35f9_02c5_5732_d1b8,
            0xd393_d20a_238c_d3b1,
            0x789d_b436_bd93_a1dc,
            0x6129_b678_a25c_04da,
            0xda0e_508d_9488_ee81,
            0x3a3c_d193_544c_b189,
            0xb755_d520_5408_5e2a,
            0xe0fb_0bf4_4646_eb49,
            0xdeda_e4f2_336f_abb1,
            0x4b71_8f8a_a27b_bc3f,
            0x6845_c4bc_e8cc_3cc5,
            0x71df_923f_6053_8367,
            0x1ce8_3c8e_e7d2_327e,
            0xe8fa_b3c8_860b_22df,
            0x550c_2a38_a94e_ae8d,
            0xa3f4_3baf_839b_0fb6,
            0x9827_e6d7_002c_76e3,
            0xecd1_3e9e_f61d_d091,
            0xc5b0_9adc_d226_9784,
            0x36a3_3219_67a6_1742,
            0xf8e5_18d8_05c7_c77b,
            0xdc68_43a7_da66_faf8,
            0xf45d_1375_20fc_acfb,
            0x2efb_2b87_1fb7_6326,
            0x65e2_9aae_4fa9_f0ba,
            0x2ce9_c60d_4fe8_52e7,
            0x1d5c_bfd9_cfec_3365,
            0x8876_10a9_d11f_e3e0,
            0xf355_8498_de96_32e6,
            0x2044_24e2_9e51_5608,
            0x21db_6117_2a64_bc9b,
            0x9212_dc02_e6a4_d583,
            0xaada_90fa_b459_6405,
            0x1e70_7353_2e8a_3e21,
            0x8173_8550_d3b8_714d,
            0x2573_b204_a074_c507,
            0x7ffa_6051_e8a5_fff6,
            0x4c36_4cd0_8371_88ed,
            0x5a7f_5755_2caf_169a,
            0x1b86_49c9_4a17_9ef1,
            0x40c8_1035_c6d3_d6f2,
            0xb080_c8a5_b846_5859,
            0xc60f_c1a7_c7ff_c137,
            0x2ce3_f51a_617a_b434,
            0x5cbc_6a8b_9e50_7898,
            0x51fd_7e99_c42d_7d1c,
            0x5793_6950_d699_3e6b,
            0xe19e_eb2b_7f21_c938,
            0x1cb4_6c86_39b2_1f47,
            0x685c_0fd1_b4e7_0162,
            0x1341_6f0f_c973_363d,
            0xb54f_e472_dc7b_cae5,
            0x93b0_ca0d_c226_f8dc,
            0xe3d0_82e3_982d_434a,
            0x6b36_10db_c1c6_7a53,
            0x98e3_1dc2_ee73_21f7,
            0xd4d4_a57f_efdc_370e,
            0xcfcf_3c32_7fb2_7408,
            0x046b_7d6e_8a01_3049,
            0x1b28_2adb_9b00_dc16,
            0x932a_e035_f0eb_003c,
            0xdd70_3d74_747f_eb24,
            0xb87b_bc85_85f7_0d49,
            0xab5d_8d4f_270d_2cb3,
            0x1f14_f290_445d_f913,
            0xe5a3_3738_a7eb_212e,
            0x6396_a803_d86f_bcf4,
            0x1932_59a5_d204_150d,
            0x0826_6961_8bf0_80dc,
            0x4cb9_91e8_8506_0fb2,
            0xc02f_7c15_18f0_b478,
            0xb854_eb7f_91d4_3b16,
            0x504c_0371_e20e_d3ab,
            0x4611_fdb0_f10f_1d68,
            0x76ae_73f9_2ad5_0a73,
            0x9e4e_e83c_7ddb_3d26,
            0x0d5a_e689_2581_37ac,
            0x49fc_7160_fb20_c302,
            0x643f_7d3e_3bff_3d48,
            0xac64_163f_8962_644d,
            0x23fe_2f8a_71d8_2ca9,
            0x1256_71bd_27f8_22d7,
            0x0aad_f376_c1d4_4cb8,
            0xb6bb_f222_383b_486f,
            0x1c9c_c0a3_728b_67f7,
            0xaed8_8ab8_4683_5597,
            0x16c9_c05b_cee9_cb13,
            0x4bfb_121f_dea4_1be3,
            0xfc1f_da40_cb7a_ce52,
            0xe8b7_ffd3_5566_f49a,
            0x13c2_30a5_adf4_0ff0,
            0x0156_b7de_272a_a031,
            0xdb4a_7155_d76e_569f,
            0x0a9b_c6a1_12b0_0f33,
            0x4c57_6868_9417_77f6,
            0x151a_5759_cd4a_1a70,
            0x1e49_bbc0_39f8_dec4,
            0xdb62_a217_6247_5564,
            0x84e4_61ca_6471_3676,
            0x53f0_1776_45a1_db55,
            0x0d32_d8cb_d874_4170,
            0xf916_dac1_560d_4315,
            0xae09_f33b_a038_fb61,
            0x0925_2fad_0a58_9a36,
            0xf328_19c2_5f79_e41a,
            0x9f68_4fde_6d1f_43c0,
            0xed77_512e_68f5_e9ba,
            0x0f5e_9711_e843_544d,
            0x3b0e_f202_80cd_8e87,
            0xee88_b721_0146_5f54,
            0x1d6d_879a_dda3_2728,
            0x2f4d_b90f_77cc_300e,
            0xb5fb_933e_4a4b_26e8,
            0x339d_56ff_d8e3_839c,
            0x6a8a_1b8d_0fae_434f,
            0xf984_9b28_3216_61ac,
            0xe594_0400_283d_06e2,
            0x314c_505a_e0ab_76fe,
            0x95d9_13da_6807_af78,
            0x491f_c008_c105_c326,
            0xe5f3_1b97_fe90_2b9d,
            0x8647_35f3_b760_71f8,
            0x7120_3231_a1cc_5900,
            0xf088_a444_a42d_9f98,
            0x0200_8a7d_7cc1_5442,
            0x0048_38c3_6e8e_5569,
            0xe893_7c23_463f_267a,
            0xff60_f3b1_cd8c_d2c2,
            0xb7b2_7f99_58b9_c3b6,
            0x7e96_47b0_ab3c_fc49,
            0x9b97_8e08_6463_ceae,
            0xcd13_ce72_5c00_30cf,
            0x9ba0_9dc3_ccc8_9e06,
            0xd64f_4c02_cc14_1e4f,
            0x1375_58b8_1835_4066,
            0x4955_1faa_e79f_e0f7,
            0x0ee3_890c_14a0_3476,
            0x2a5f_a8ac_7b2d_3cd6,
            0xabae_6f31_a0f9_4ce5,
            0xb9df_c671_d9b6_4671,
            0x6316_a1ae_f266_c70c,
            0xcbe1_328c_4a93_1f15,
            0x128f_ddd6_bae3_6eb9,
            0x4d9c_f2e6_e585_51b7,
            0xe340_d98e_d484_5adb,
            0x3421_c2f7_64c4_895e,
            0x5a30_48ac_3aa3_1319,
            0xbfe8_9ff1_0a0a_120a,
            0xcf30_77d6_bf7a_0d8c,
            0x307c_6fec_d800_8ab9,
            0x0c35_bc4e_86d7_8c86,
            0x4fa5_7bf2_2e8d_d433,
            0xc69d_f9c0_a1bd_3880,
            0x794a_d057_eb94_6eea,
            0x74cd_1c28_a971_2c03,
            0xd37c_df57_b8d6_51bd,
            0x74f7_db11_6f25_abe0,
            0xd640_377b_658a_6d91,
            0x54a9_32b9_565e_e33b,
            0xb4a2_9ce6_8379_52ce,
            0x5ee5_8161_62b7_308f,
            0xcc47_1b54_5f4e_b067,
            0xbff3_db64_de5d_b1a1,
            0x89f1_0406_672f_d2e6,
            0x8477_b6de_2f8b_527f,
            0xb890_4129_d9a3_973a,
            0xf717_ef1f_1ff7_8baa,
            0x91b1_ebf5_e935_1c9e,
            0x960c_771f_51ac_29d5,
            0xa307_094b_a277_6dfd,
            0x7e6c_18b6_8000_803f,
            0x4d31_f402_f867_178d,
            0xf3df_231c_a1ca_2c7c,
            0x7963_b0fb_0b78_5f80,
            0xa115_9fdf_6521_c264,
            0x1290_e55b_a9bd_e549,
            0x13c9_12f6_4a2a_4774,
            0x8fb6_c520_0edf_6c9f,
            0xcb23_7c95_7caf_773c,
            0x8781_04f7_5d43_b560,
            0x6f3d_b6ce_82df_211c,
            0xdaa6_653e_846f_c383,
            0x305c_6554_a60b_8666,
            0x256c_a93d_2c00_d3d8,
            0x3751_74da_3766_4379,
            0x791f_94ba_3766_4d37,
            0xeea0_ac2a_97e2_c316,
            0xa3fe_c663_7ecc_ed3d,
            0xbd82_7021_d53f_e200,
            0x9168_16f2_051e_4223,
            0xb5f3_8ebf_d20c_2362,
            0xb6b7_75d8_39c1_7ca2,
            0x0f7f_00e0_5302_0b2d,
            0x7a8f_dc92_caf7_2144,
            0xdb19_a009_4ac4_4350,
            0xf21b_6624_bbfd_1d73,
            0x5b67_7343_381f_d531,
            0x71de_a5a1_1033_ab16,
            0x3a09_0f96_ad71_04b4,
            0xc9bd_6763_8fa9_efe0,
            0x6d45_7f51_5d39_5157,
            0x2cc6_7ab7_23a7_450c,
            0xaa45_e811_547e_6e7e,
            0xbcf5_32e5_5b24_4f61,
            0x9141_648f_d63e_5d07,
            0xe87c_f733_799a_d716,
            0x3e97_2a12_5683_08c2,
            0xd7de_2d7d_868d_c583,
            0xa1c2_575b_ef6d_1f94,
            0xb068_b879_32e0_6f2a,
            0x852f_c000_1d23_65ce,
            0x980b_d664_8c2e_e57e,
            0x1acf_ae88_5645_48c0,
            0xca67_a2c2_cdfa_c2d1,
            0x2fa2_e873_0f5d_2350,
            0x0e08_a5fa_c69a_1591,
            0xe242_f739_c966_5569,
            0x15ad_32f0_d913_5e3d,
            0x3c43_4c16_b945_e40b,
            0x9000_e1fa_f218_fc64,
            0x2fec_bb2b_213d_6e76,
            0x1503_b5cb_9e6f_8971,
            0x3db8_3e21_3203_a30d,
            0xce08_73d4_3c2e_5211,
            0x3d38_be5b_0938_7e2b,
            0x7dd8_e23b_6316_c34e,
            0x221c_0431_ffc8_6320,
            0xc4f2_9e3d_6777_33c4,
            0x5cd7_3e12_8450_e94e,
            0xd1ac_c654_fd55_5a27,
            0xca62_11f0_3940_a1ae,
            0x0c53_1719_8ae6_a874,
            0x7045_0f97_200c_f50f,
            0x03e8_c9db_b9df_94ac,
            0xa2ad_3c91_f54c_f016,
            0x2b8c_9351_d1ef_292b,
            0xa2ba_9f87_189b_078d,
            0x7290_4559_1d4f_72a5,
            0xe6af_3f2e_c8cf_cc4f,
            0xc553_06c3_48fe_c2ec,
            0x0da6_4fa3_31f1_f0ab,
            0x0c9e_f1bd_82e8_7d11,
            0x787f_886a_06fa_a8d0,
            0x8e60_f6e8_6eaa_ebc8,
            0x0af0_cbe5_9ff7_276c,
            0xfef7_d4d1_1c04_0662,
            0x2eb3_d47b_d458_5ac4,
            0xf3b8_3703_fa53_a616,
            0x737a_cac5_a2cf_f3a7,
            0x53b7_651b_c86b_b2e0,
            0x98be_24b9_e22c_dcae,
            0x923d_4667_ff4d_0493,
            0xf290_68c0_dbcc_d915,
            0x5a0d_cd68_5538_a740,
            0x466e_71b5_f26d_f003,
            0x8fef_1991_ec94_2b99,
            0xd399_d55e_fa63_542f,
            0x600c_80ec_8a17_f30e,
            0x36f7_5265_99f7_a8a1,
            0x4324_4ef5_b249_10af,
            0x279f_5ffa_2e73_f5b7,
            0x3021_cd61_b6f3_ddad,
            0x3d67_7f27_a544_0616, // highest
        ]);

        #[rustfmt::skip]
        let num2 = BigNumber::new(vec![
            0xa164_1bd2_ce7a_b78f, // lowest
            0xbd15_71cf_df71_3698,
            0xeae2_8a9d_ba9a_ad72,
            0x3ce7_5a08_b4ee_e2cf,
            0x77f2_dd10_0288_2c20,
            0x8130_ed47_b52f_2c9a,
            0xdbde_bdba_02ce_e3d6,
            0x9c12_0e23_d46b_a096,
            0x1dfa_1ff8_f4ee_a4dd,
            0x292c_ff11_b383_cf6a,
            0xc43a_ac54_eb19_f044,
            0xdf60_e1af_c0ec_7acb,
            0xe932_1405_6fc9_b0d7,
            0xfd01_d631_a2a4_6e25,
            0xdf1e_3223_c762_ad7e,
            0xd448_8c08_65af_fa8e,
            0x90bd_30e1_736a_b5af,
            0x1df2_97ab_046c_9ff0,
            0x2977_805a_4a54_4b1f,
            0xedba_aea9_ccf8_aa58,
            0xac0f_117e_7937_5256,
            0x1655_b64c_97cf_11b8,
            0x9481_d6ef_9b16_99b0,
            0x0a19_9095_a656_f83d,
            0xd2e3_cf09_3325_b005,
            0xcc8a_862f_73b6_a885,
            0xea3b_758a_5133_253e,
            0xd77d_fbef_e68b_224e,
            0x3962_5ce7_bef1_8547,
            0x7e2a_db5e_339f_9644,
            0xa161_8c7b_9784_8ed4,
            0x6f57_5ad5_95c6_4ac2,
            0xd3fc_4800_369e_b17b,
            0x58b0_3e0c_165c_33f9,
            0xeca1_5487_4181_e179,
            0x9d1c_ac6c_0b12_510f,
            0xe6f9_20eb_5b5b_6d3d,
            0x1d36_9184_d3e1_0ae0,
            0x15b9_f3af_2bf2_c44f,
            0x1b76_fddc_3995_6df5,
            0x0a9f_1222_72fd_710f,
            0xb490_7dde_8773_197e,
            0x8ac8_55a4_6b41_3af0,
            0x8830_bd2d_27be_226f,
            0xd6fe_dbb4_f71b_1776,
            0xd396_cad8_582a_c0c4,
            0x6898_74cf_7703_96a4,
            0x9689_987f_ca6f_c853,
            0xf980_912a_2807_628e,
            0x03b4_a9c7_9f35_3c1a,
            0xece0_744c_7c66_3d5d,
            0xd6b6_59f1_a756_71a0,
            0xd028_474d_bf97_b45a,
            0x0bf3_3d23_898e_1fc5,
            0x8db6_f5d8_7319_eb7d,
            0x6e83_df1d_09cb_d69c,
            0x67e9_1a26_b574_7a2e,
            0x7a8f_773e_404b_5ff1,
            0x1aa8_68b6_e535_957d,
            0xaf66_c66b_96fa_57f1,
            0xbf71_c626_c5b3_7619,
            0xa34a_d665_437f_d1aa,
            0x1880_d230_9508_eede,
            0xa6fe_905b_fc18_0133,
            0x8c14_3b6b_4254_ffe3,
            0x5f6f_ac56_7ba1_4f61,
            0x128c_554c_a7fd_33e1,
            0xddbb_efb6_c302_5930,
            0x4255_ae40_d211_3254,
            0x0e0c_bc44_55e5_c98b,
            0x6fcb_549c_b59a_b3be,
            0x5aaf_aae7_cde4_4c69,
            0x3208_7922_3b77_8b45,
            0x6d27_bdc9_1e82_a6d5,
            0xca4d_c939_7341_ff42,
            0x3316_4a06_e8e4_aae1,
            0x114e_2b34_243b_4031,
            0xb860_f8b2_19b4_890d,
            0xf5bd_b9e9_b730_5013,
            0x02b0_2875_0d70_4c85,
            0xdcc0_cb58_94e8_a407,
            0x2ca5_2d44_8ad2_5be3,
            0xf303_3d6d_cca6_61e8,
            0x969d_db64_092b_e4da,
            0xa26b_d561_ce08_39ff,
            0x311f_6301_9f54_aa81,
            0xf7c2_82f7_4c43_dc2c,
            0x0b10_5131_22cf_a89a,
            0x3f8d_3849_4ca4_5dd1,
            0x01dd_563e_dbbe_94e4,
            0x0754_80bb_b642_e3b6,
            0xeb82_6dbb_052a_f857,
            0x1c98_df7a_fb3f_d3d8,
            0xc31f_dd2b_5ad3_e2ea,
            0xd5b7_1c29_4e4f_6373,
            0x51fd_890d_488f_de74,
            0x71b6_7c0b_a65d_246f,
            0x5dcb_47c0_d59a_e086,
            0xb130_43bf_5b8b_6efb,
            0xc8a1_df3f_e168_f998,
            0x375f_97e9_206c_61a9,
            0x9df8_301a_519d_e722,
            0xc89f_8955_6ce1_15d2,
            0x3f3d_04ab_996f_5c91,
            0xf15b_1fa1_ae15_2091,
            0xa304_2e02_3d27_8502,
            0x0bdd_4acb_0248_55a9,
            0x4f0f_ec6a_d532_a6d3,
            0x0532_5ede_a4f2_894d,
            0xa8f7_7f51_2ea8_af81,
            0xde11_30c3_f52f_37c5,
            0xa517_c3d1_422a_b32f,
            0xeee8_ad91_d865_024e,
            0xe0f3_7bda_ad78_5be9,
            0x8821_0d5d_1f0f_a368,
            0x9a4f_5f8c_ec76_9cf2,
            0xfe88_25e0_0079_217b,
            0x81ee_c5db_b748_a0ea,
            0x0b83_8674_5924_7820,
            0x3edd_189a_dafb_23a3,
            0xb116_de20_af3f_ac29,
            0x05d0_15f6_a29e_d8a3,
            0x4c5a_699b_4634_3263,
            0x759c_34f6_0a04_fa0b,
            0x019c_c305_9f0a_fc55,
            0xf85b_4547_76fa_d5ea,
            0xfe07_333f_b6c3_1c80,
            0x47bf_f43a_45cb_bd88,
            0xd32a_3b46_8f32_6185,
            0x6745_ab04_69d3_7335,
            0x65db_ff61_719a_9e09,
            0x4325_3ef7_e5dc_64e2,
            0x1bc6_8328_e5c1_4c87,
            0x1577_960a_17a4_0f76,
            0x1f5e_3a54_e096_1399,
            0x38cd_c9b3_b426_7b04,
            0xf30f_bc7b_df60_e4a5,
            0x8d6c_618f_ad25_c5ae,
            0x9844_afa5_9565_c927,
            0xc3ed_eca4_4cd4_31eb,
            0xeba9_831b_bb5a_2f55,
            0xb8fe_5371_7458_386b,
            0xe699_d32c_f111_32b8,
            0xd340_e249_7699_dc86,
            0x2594_ae81_9e3f_e513,
            0x05fb_b0c3_bd8e_dd13,
            0xcf70_5b30_602b_6674,
            0x390b_c19f_65c6_4c31,
            0x5154_b659_82f5_3e60,
            0xcc8b_1bb0_9a6f_011f,
            0xdae5_cf8b_6289_9912,
            0x003d_fbd0_adc1_681e,
            0xbf1a_7542_8789_045d,
            0xb2b7_d3aa_96b2_2be4,
            0x1151_a9fa_51ca_92bf,
            0x8810_a538_2f95_efd6,
            0x9c74_7ea0_8612_fa09,
            0xe507_0f59_127e_42ed,
            0x9423_6151_1b65_2f8f,
            0xee56_038d_4a9e_c046,
            0xfa61_52da_50a0_730a,
            0xab50_a6e1_8c3c_3e7f,
            0xf174_106f_947c_002e,
            0x963c_3d7e_8e2a_1e3c,
            0x26b8_5cd5_81dc_3083,
            0x910c_3c68_97c5_8ed5,
            0xc352_d5eb_6b07_7285,
            0x8e2a_7b5f_4627_2e26,
            0xd3ed_3568_8ce3_c5a7,
            0xfb91_6a38_9e49_0905,
            0x2b42_a70b_5dea_066e,
            0xa20b_3326_70be_b137,
            0x5d37_92d6_a27f_3e24,
            0xd8e3_8c6d_6f72_c560,
            0xbd3c_1ba4_7cca_c851,
            0x493f_8d0f_fa9c_1cbf,
            0xaed8_b4b0_13ff_359e,
            0xcff4_29cb_76b4_f262,
            0x3da8_9c5a_9f4e_2740,
            0x0502_f683_46b2_650d,
            0xef95_32d8_7625_9d87,
            0x9ca1_9a7e_4325_c802,
            0x47ef_5657_1f4e_d87d,
            0x3122_1ada_7fe7_d4e4,
            0xc47b_645b_09b4_2e48,
            0xa319_383d_d70f_be49,
            0x3fd3_5d22_e4cd_de07,
            0xf411_0a2c_b175_3eb2,
            0x4506_988e_7a88_2bff,
            0x872c_40af_0078_dfc1,
            0x8284_cf1d_48a7_5a20,
            0xe019_583e_eea8_04b9,
            0xe741_3fd3_8e73_d78b,
            0xcb51_fbf4_57d0_221e,
            0xf8c0_622d_63ce_0b16,
            0xbc33_245a_9369_6b4a,
            0x1d02_bd52_d423_cde4,
            0xef01_a7d3_e398_4a41,
            0x8714_9e11_eb79_0078,
            0x461c_421b_1406_0da9,
            0xaf90_eb42_f002_2585,
            0xae38_3298_6890_5fe0,
            0xb5cb_9a04_1bd2_bbae,
            0x2219_7ebe_9c84_89a0,
            0x765e_9a5e_1504_f2ce,
            0xe432_97bd_5d20_82da,
            0x8fff_e8c6_35e7_ade1,
            0x256c_c29f_7d4f_6a0a,
            0xa079_b862_d939_0048,
            0x53f8_77c5_ab58_7560,
            0x8750_eabd_a44b_9f2c,
            0x7aba_2a5e_418a_624d,
            0xf8c0_ce8c_8720_6d00,
            0xa75a_c42c_3729_467f,
            0x3bd4_1422_8be1_1b9f,
            0xc6b7_8eb4_f9b6_bf1a,
            0x0ea1_6b88_0d0f_7e6d,
            0x1796_9160_268b_c434,
            0x91ed_59a7_68cf_ab8e,
            0x0d62_c1b7_41fe_30d1,
            0xb129_7ea7_733d_d9ba,
            0xca95_4ad5_9817_59c3,
            0x4909_0642_e78d_8d11,
            0x1c70_4c1a_8662_58f1,
            0xcc1e_0bb8_e445_cd0b,
            0x4580_e57d_3519_00ad,
            0xe45b_fd62_24ae_f27b,
            0x3a80_7a91_acd4_b278,
            0x89ea_11cc_e2d5_bc4e,
            0xe531_6b5b_8328_21e2,
            0xada4_bc4d_10cb_1ed0,
            0x873e_6b2c_06ff_aaec,
            0x8014_b7c8_ab08_561b,
            0x86ab_8e0a_713b_2195,
            0x74d0_437d_8f2d_b8b0,
            0xf1d6_563b_6219_730c,
            0xea0c_9c3e_7bce_67bc,
            0x3e92_a9b6_8208_2a98,
            0xe084_40ae_d0e0_d127,
            0x8132_d131_5982_4676,
            0xcd9f_6603_702f_4a99,
            0x2fb9_179c_313b_0327,
            0x7baf_42a0_edc7_dfe6,
            0x7994_072c_c39c_0569,
            0x061e_bcaf_04bc_032f,
            0x40d2_1630_9b66_ee98,
            0xdda9_ec17_7739_6af4,
            0xab58_57e9_9466_2d54,
            0x4ca5_e593_82d3_3eef,
            0x50cb_d998_5353_f6d0,
            0x0018_5238_1a48_8d1f,
            0xd658_951b_ff3d_acca,
            0xfe38_fc35_0cb7_9854,
            0x8d07_5aeb_4ebf_b917,
            0x9fe7_8840_87be_ab31,
            0xf74a_d747_9049_f107,
            0xf4a4_32a2_c9cd_6bd3,
            0xa8b2_f5e7_092e_717c,
            0x7918_f133_15ad_d355,
            0x40ea_12ff_c697_6c38,
            0xb200_5179_65c1_1ccc,
            0x1c82_138e_bac5_a44b,
            0x5247_9cdc_f6a3_6952,
            0xb288_ff3d_38ad_0131,
            0xb877_5ae0_58ac_a06d,
            0x184a_f34b_cc72_f969,
            0x7bfe_ddb9_12fd_69f2,
            0xce6f_f085_394b_be71,
            0x8a35_2894_6ccc_51ac,
            0x02d5_e3bf_ce6b_3b7d,
            0x3668_06c5_922b_25b8,
            0x294d_ddd5_c38b_a19e,
            0x2707_58d9_a7a4_e645,
            0x1098_0a7b_e07a_7bda,
            0xbbd5_0a7e_a086_5f1b,
            0x6cc8_3737_dac9_0548,
            0xbca3_0667_47f8_79c1,
            0x0bbb_46a7_d74a_d743,
            0x3375_b23b_3189_b803,
            0x3a27_f8f0_b988_a16b,
            0x03db_3cf8_116a_1484,
            0x144d_fba2_c982_c784,
            0xf0e1_fddc_c951_d31b,
            0x324c_fd9e_191d_478e,
            0xc442_5adf_79f8_9025,
            0x6cca_5b82_a8e0_1fa8,
            0x6864_c00a_3403_bc96,
            0xd7f0_c3bf_1a54_44d5,
            0x30d5_6364_58e2_c125,
            0xeff0_29e7_b156_4369,
            0x8240_8c5b_e969_e62c,
            0x2224_1117_1a03_ab55,
            0x3255_070e_1d8a_3ed7,
            0xe3a6_cf8e_abfd_8ce6,
            0x0160_3308_1a84_177d,
            0xc8e5_bf16_9f43_509c,
            0xc098_9a48_1b35_009d,
            0xc638_dfd8_af37_a058,
            0x7185_2da2_747c_09a4,
            0xeb6d_c987_acc2_1f30,
            0x9cb7_baf8_d272_a621,
            0xfeaa_1af8_a494_86be,
            0x699f_4bd4_c361_c56f,
            0x4ab0_fed6_9f42_75de,
            0x84fd_f5b8_b033_ea1f,
            0x3590_8d29_ac25_f5d1,
            0x729a_0825_2634_339b,
            0x6990_5cc3_01d5_581d,
            0x9f29_69a0_fa9a_e2de,
            0x6f7c_b579_5fca_4971,
            0x29ab_7fca_9dc3_b779,
            0x2cb1_0978_86b6_325c,
            0x9dd4_3ae7_5d47_5e1d,
            0x4802_0f51_5436_9706,
            0x56a9_cb65_36aa_899c,
            0xd721_b73d_0990_a596,
            0x1c78_00f3_fa0a_a81d,
            0x5322_c0f3_e4b7_35d8,
            0xb6c2_9421_f236_817f,
            0xfa69_6da7_c07e_04c7,
            0x10dd_e3e9_e1df_c58c,
            0x5fce_5645_95f7_7f72,
            0x20d0_3c48_cb71_40ed,
            0xab94_d02b_bc3d_187e,
            0xe5df_3acd_84a4_c25e,
            0xc631_fbd0_ccaf_105c,
            0xce4b_f7c2_8533_7c0f,
            0xbb23_5e3f_9179_cf05,
            0x3fd4_589a_24d0_e35e,
            0xe42f_5d9e_3310_ba58,
            0x0988_3e6f_98c5_0f7e,
            0xbe88_f1ab_351e_50fe,
            0x9f9e_b539_ff4b_5bfa,
            0xee8d_5914_da0b_2af2,
            0x3499_8cea_a432_8241,
            0xd4db_6f2c_1406_020a,
            0xe853_02d1_d783_56f5,
            0x54a5_101a_a9d3_9f48,
            0xc4ef_ea9e_ded7_3a0a,
            0x9068_1631_a8b6_1382,
            0xc5aa_432a_d6d3_4605,
            0xc169_92a6_458b_454c,
            0xa959_88ec_54d7_61b0,
            0x66ad_ef88_7bee_8f21,
            0x49f2_14e2_dd87_cd98, // highest
        ]);

        #[rustfmt::skip]
        assert_eq!(
            &num1 / &num2,
            Some(BigNumber::new(vec![
                0x4bb5_f7d2_9bd9_caff, // lowest
                0x5218_1b6d_8587_7158,
                0x6d16_26bd_bcb3_e198,
                0x9b44_47cf_a8cb_c426,
                0xdd8d_f2a2_ee11_3ba4,
                0x64d4_0744_a735_5a7a,
                0xa672_c44e_fcdd_3928,
                0x2832_6bb7_d9cb_a935,
                0x354b_3de6_39c7_0849,
                0x393e_73e4_c3bb_6e3f,
                0xa764_464f_52f5_3fdd,
                0x3b53_612e_3831_1aa3,
                0xf364_46e1_d2a1_ff45,
                0xd8a7_21de_fcb3_9ffd,
                0x2670_4626_b298_b04a,
                0x7a4e_547f_83a0_fb3a,
                0xb724_b676_8668_8588,
                0xfd2f_7efe_ed9b_3339,
                0xdfec_5469_9c57_c378,
                0x5b43_136a_76b1_4927,
                0x0804_a1c7_78ae_680b,
                0x0ae1_1c01_301b_d42a,
                0x0236_1018_3150_7efd,
                0xc696_5699_e380_e026,
                0x4095_6377_3608_7f11,
                0xdb46_34ca_5794_0289,
                0x49de_dcd9_f748_68d3,
                0x677b_b16b_dc2d_04cb,
                0x9ded_2593_49fb_8978,
                0xe411_191a_4640_fc53,
                0x608d_e184_7b98_0421,
                0x60b8_079c_2c22_e720,
                0x93b7_bf05_f635_f52b,
                0x6f24_f279_57fc_f3af,
                0x69ae_601f_e577_9a7f,
                0x17bd_8ede_644c_dda7,
                0x20f3_696f_456f_25be,
                0x4ee6_81aa_cbf9_dd4f,
                0xd823_e92e_8751_73ea,
                0x69d9_bde3_7ab8_50bb,
                0xacfe_37ff_2981_19d9,
                0x6b2b_6308_2e3d_495a,
                0x7ebb_4355_21a7_82fb,
                0x37ab_1278_bf0a_20d2,
                0x86a7_1925_968f_92dc,
                0x2169_f4d3_d80b_6d17,
                0x93a9_3ec0_e3e7_60a5,
                0xf8c9_a5e6_17c7_7ff9,
                0xbd7a_3e20_95e7_9e11,
                0x6737_9546_a6c6_0872,
                0xa06a_0234_4b08_ad50,
                0x3ac2_1b10_b03e_1da8,
                0xa573_ad6f_e38c_66b2,
                0xd7cc_6807_ee87_2535,
                0x58b2_79c8_bf23_5fdf,
                0x55c8_f554_90ba_2a2f,
                0x7155_3a4a_aa37_1abe,
                0x3848_19ea_8144_b27c,
                0xd535_b456_e96b_9a81,
                0x4e06_e950_5318_8555,
                0x3ed4_0bea_415a_1a92,
                0x7512_0983_8a98_b475,
                0x9bd8_36ac_323b_6045,
                0x4d8e_a2de_15c5_e3f1,
                0x3c1e_e6f4_37dc_0b5f,
                0x86fa_4e06_c056_70de,
                0xab72_1f92_e39d_9fb3,
                0x7517_cfb1_1112_fe2c,
                0x7186_8f38_97c5_9ce0,
                0x5612_8448_e6bc_f38e,
                0x849d_81e8_ef24_0609,
                0x8fb2_0a29_4540_272b,
                0xad9e_ad31_e589_0b41,
                0x65e4_a7ee_e25c_8213,
                0x8ed4_116e_e25e_aab7,
                0x699c_ecba_4fcf_8471,
                0xe0ad_3f79_a411_7f28,
                0x42d3_0253_8e53_5086,
                0xc9c6_aa43_58d7_312f,
                0x3fb1_6ee6_d00a_6358,
                0x8722_5258_6ef2_cf01,
                0x949b_f476_6963_a7f9,
                0xa0c8_c49d_b4bb_c210,
                0x1155_786b_ee82_4d3e,
                0xb2dc_9ada_393a_1090,
                0xf36a_5b01_e7d7_017c,
                0x1e25_9ee5_9d1c_528e,
                0xd602_d237_d4b1_70cb,
                0x0a3c_98b8_2951_456b,
                0x61a5_694f_bda8_2116,
                0xd203_9afa_0a58_74bc,
                0xf1fa_3e6a_997c_c2f8,
                0x3116_a163_419c_48a6,
                0x3094_661d_82d4_ea12,
                0x4397_87fc_b30f_d3f6,
                0x9df2_528d_28ba_0ebc,
                0x8244_3f2d_9c3e_42c1,
                0x0eb8_aab0_97df_a681,
                0xc5e7_7028_c631_1099,
                0x9514_383a_5676_770f,
                0x6ecb_4c01_f79c_0524,
                0x1aad_e3d4_9fbe_3570,
                0x2a59_8b34_84ff_754b,
                0xd815_eb76_aea3_52af,
                0xadcd_1b58_4b39_e22d,
                0x32b7_1be9_c80e_5f29,
                0xd463_0cc3_492b_b9d5,
                0x8d54_fccc_5b3a_e833,
                0xdb69_0d4a_f944_33b3,
                0xdd89_3f7f_f461_6234,
                0x9b84_e6ce_da0a_52ce,
                0xaf2f_454c_54bd_de35,
                0xbeca_af87_7b9f_0f92,
                0x6fa4_9331_8a2e_12d7,
                0xaacb_16ed_b0b9_5adb,
                0x7c6e_fe3f_de3d_3a53,
                0x5c00_b0c0_8de8_2c68,
                0xe1ab_5d81_286e_7768,
                0x9716_b390_588f_4c79,
                0x6744_f29a_6e14_184a,
                0x5c62_5d90_6e09_ac60,
                0xf7c2_ff37_458a_c510,
                0x827b_99c1_de9e_5ce3,
                0x2447_3180_7f63_08ed,
                0xfd90_3a2e_2f47_c6bb,
                0x05bb_671a_108a_84c9,
                0x9867_4167_4ff4_2974,
                0xdd67_030e_852a_b540,
                0x9a7f_d651_ce14_e23c,
                0xa993_3066_82f4_b570,
                0xbb40_b05e_fd0f_66bd,
                0x96f7_9329_03e8_079c,
                0xa751_73c2_fd13_e6a1,
                0xa4ef_4d92_ee7e_500b,
                0x3a21_54d7_4168_e087,
                0xca7e_4e5d_b074_4375,
                0x3bf8_7281_8928_2c3f,
                0x62d7_7cc4_599c_0176,
                0x0165_fcee_6020_397c,
                0xd775_48a8_96a4_149c,
                0xf1e8_89fd_3eea_05c9,
                0xb152_b1f9_77e9_feb5,
                0xbcfa_36c0_8739_7e26,
                0x8042_05f4_05d7_d5d9,
                0x8f3c_0935_96d2_988d,
                0xb422_5673_94b3_7311,
                0xc522_9653_61f5_9f0a,
                0x100b_8777_26db_a6fa,
                0xa795_b41f_2cea_5ebc,
                0x93a2_e4f1_2785_4dbc,
                0xe9e1_7588_8383_10d0,
                0x5cec_c8bb_58f9_172c,
                0xf14b_9135_c72f_db19,
                0xc9ee_34bc_587a_4fcb,
                0x8ced_c754_ac75_0e89,
                0x668f_e724_77ab_2678,
                0x809c_1a08_280a_4f07,
                0xe79e_8d12_045c_122a,
                0x658c_4208_d9f8_16d6,
                0x4b46_ebf7_5e3a_58e7,
                0x53a2_e119_4b98_cb85,
                0xd736_5ebe_f973_db7f,
                0x4c2f_c895_3127_594e,
                0xa20b_884b_0ae1_c452,
                0x7d58_fae8_3d04_e2df,
                0x7297_985d_ef7f_954a,
                0xdd3d_00b4_4f32_de1a,
                0x0df5_4d49_76d4_f0cd,
                0xef08_c328_fa18_ac47,
                0xa561_bc62_c699_64fc,
                0xc958_d896_e7c8_6eb8,
                0x3812_2ff1_520b_01c8,
                0x4873_c264_6a34_561f,
                0x4989_dfbb_6a67_6916,
                0x5c6a_db81_1118_43b4,
                0xb05b_d8a3_e62c_0c43,
                0xe120_773a_01ec_66a9,
                0x72ea_eebe_39b7_1f04,
                0xf5d5_96f1_da9c_e51b,
                0x3133_6de1_751c_cff8,
                0x1d13_b557_56d9_0384,
                0x43c3_ef6f_f3ee_be8a,
                0xac58_55bc_7621_45a8,
                0x9a7c_cc8f_7ee2_9c40,
                0x0acb_1544_0811_b6b9,
                0x089b_988f_63cd_00df,
                0x29fd_d451_39cc_d599,
                0xa359_30d8_9ed6_16b0,
                0xac3b_05d8_63cd_da8e,
                0xf014_dce2_210f_f4c2,
                0xebf5_b019_622b_9706,
                0x2214_52b2_07dc_de57,
                0x6386_4b46_69e9_13f7,
                0xb721_cfd3_2b45_02c9,
                0x33b9_2342_9b49_97bb,
                0x0a5c_cfb9_b05a_c3c5,
                0xf113_e1dd_8f94_e71c,
                0x19a2_160b_949e_d1e6,
                0x6a51_b357_63d1_6b52,
                0x6800_be41_8a46_1772,
                0x81ed_d720_82a5_628a,
                0xbdf3_16c8_b988_0c82,
                0x209d_6146_579d_8b7c,
                0x56c0_8294_1e10_8ffc,
                0x919d_bf56_6f7c_f123,
                0x5f61_eed0_e8dd_609c,
                0x69c9_f324_7543_167a,
                0x114b_3bd7_6e26_d647,
                0x54dc_6c77_b624_6126,
                0xa2e8_b4e7_cf27_f9a1,
                0xb00f_1b32_1fd1_f266,
                0xe6a0_8178_100d_616d,
                0x4b54_c8f7_3ea6_6d7e,
                0xef22_2904_0b1f_3402,
                0xa013_5718_4a52_94e2,
                0x5faa_875c_5762_2769,
                0x6abb_8c7f_5a60_85fc,
                0xe6aa_f466_a0f0_ce49,
                0x2588_da51_559f_e202,
                0x4157_2f1c_042c_2126,
                0x8bb7_c949_fe81_4caf,
                0xd2ea_8c08_975b_bd83,
                0x2333_6745_7f6b_0116,
                0x19b7_4082_df40_df4a,
                0x8fe6_6559_f12f_bd6a,
                0xe3fc_cc8a_ae89_a1d5,
                0x0421_82fe_e135_5029,
                0xe2ac_196f_0f25_38be,
                0x67c9_8c85_0ae2_591f,
                0xb443_af5c_a6c5_567e,
                0x60e3_b3a3_4b9c_535d,
                0x68d1_92d2_456d_cd6a,
                0xdeea_9c30_56bc_d8da,
                0x783d_442c_d48e_9834,
                0x2eb8_3b75_4d3b_0375,
                0xf170_48d6_88b9_5a63,
                0xa8b8_78d9_52d0_3221,
                0xd30c_b6bb_60a3_a725,
                0x94ca_acc5_54b7_a3b4,
                0x8183_c6b9_87cc_85ff,
                0xe46a_f7b4_6c0d_46fb,
                0x1f9c_cb39_b862_b0d1,
                0x54d4_be1b_3cc0_75ea,
                0x975a_474c_0f0d_ac6c,
                0x2beb_0a7b_6dc6_e4d9,
                0x1521_fc3a_9104_2583,
                0xbda9_faec_c645_edf4,
                0x5fd5_9763_4477_b779,
                0xef74_66b3_3c4c_bb5c,
                0x637e_85cb_2689_a931,
                0x3961_2569_78da_293c,
                0x93f4_135f_5d46_f947,
                0x8d07_6865_c041_b147,
                0xa542_8b6e_a528_eb10,
                0x0401_abd1_bf90_8312,
                0x1ee0_6ff7_1ae8_0d79,
                0xde09_9683_a6f1_6f59,
                0x9e16_196c_b088_0e5a,
                0xb781_94be_0e21_8ecb,
                0x4903_c217_ba50_95cb,
                0x6ca7_90aa_e771_60e3,
                0x5e4b_d5fa_e078_6b04,
                0xe08f_8480_8ed8_1994,
                0x14b8_bd23_5d80_e664,
                0x61f4_2958_113e_734d,
                0xf66b_d4c2_1af2_4e82,
                0xd83d_a4e0_a367_428f,
                0x5d0e_51f1_7988_24ff,
                0xcff7_b7ca_55f4_08b0,
                0xcc21_1f43_d184_1222,
                0xd943_9cb2_f6c9_54b4,
                0x988f_9272_c944_e23b,
                0x408a_ae5c_2f88_dc1f,
                0x18f1_33f9_fb91_d797,
                0x2924_287f_0e25_6305,
                0x2dfe_fb94_4759_8bf8,
                0x55df_1bd4_286d_631a,
                0xe258_8d27_165e_73d2,
                0x8537_9aac_02c6_ca32,
                0x0393_f170_6273_4f19,
                0x8604_5d1f_0068_bdd6,
                0x3eba_c6b0_70dd_e92f,
                0x62cc_a94e_1325_9207,
                0x9e6b_b113_8225_7eeb,
                0x77fd_0915_ade9_2fcd,
                0x756c_a685_4563_b1fe,
                0x2d31_850d_40ad_7516,
                0x28c7_ceb9_e5c4_b09d,
                0xe4ae_3a3e_8579_1ed9,
                0x9fed_f27a_539a_6bed,
                0xca6e_1da8_ecd5_059a,
                0xb3c3_ba13_20ef_b167,
                0x75bd_0fd5_e2a6_fe37,
                0xc55b_9c99_2271_f93a,
                0x5d14_342b_4941_6a5a,
                0xb19f_cc40_ded2_91a0,
                0x8cdb_1eff_a9fc_8e2e,
                0x73aa_dfd2_7f95_bac4,
                0x13ce_892c_1ab9_4eaa,
                0x2f97_3943_e4b6_aa40,
                0xf64a_f703_012e_0d48,
                0xa4aa_893e_b860_9278,
                0xf004_dc69_fbbb_6658,
                0xe3a0_5e47_b7bb_bce4,
                0x8df7_4a57_ff2d_8e37,
                0xf40e_2a4d_370d_3cbb,
                0x7ef5_d59b_5da6_57e1,
                0xeaa0_f00e_a7e2_20fd,
                0x0e23_cfe9_5ee7_d192,
                0x7dd2_b4ec_322a_576d,
                0x80e2_13e3_2f12_e734,
                0xd521_1068_b6cc_1063,
                0x9e9a_64cb_8a47_fce0,
                0xc919_844c_74b1_a22b,
                0x827a_5f3e_b544_5099,
                0x520f_c7fe_ef8a_eaf1,
                0x8a8a_7a19_31fd_739d,
                0x536a_5beb_8070_d9d2,
                0xd278_10f4_eca8_bece,
                0xadf6_c34c_7191_809e,
                0xfbf5_6136_b67c_d36b,
                0x0de8_899e_4328_21c7,
                0xa012_22a0_ce24_8a41,
                0xb48c_c773_ed31_94a4,
                0xba42_f162_88b3_5cf6,
                0x5833_aee2_1aa0_2963,
                0x93ad_dfa5_5c00_8eea,
                0x301f_8868_4037_dbee,
                0xbfe8_a7f7_8817_1897,
                0x5cc1_5eae_ac69_2866,
                0xfa6c_d051_7af6_1b3d,
                0xe609_77f3_d5e0_cb9b,
                0xb74a_10f8_6db0_9369,
                0x76cf_f8ae_0962_a4aa,
                0xa8d1_e40c_6488_ec86,
                0xe0ba_f988_21e9_c79a,
                0xe082_2cfb_dc85_d364,
                0x42be_9bd0_1b69_ad04,
                0x0fb1_63a0_1ea0_8356,
                0x7edf_48e6_372d_9278,
                0xd132_31bc_fbc3_bdc1,
                0xaaa8_4024_636d_5e47,
                0xe55c_eb14_5fa9_55cb,
                0xf4ec_7c42_6ac7_1630,
                0xef40_4478_a2ab_4d9c,
                0xd32f_9447_7586_5866,
                0x904a_bf33_a5b8_f4fa,
                0x0c30_360c_711f_87db,
                0x61d1_03b3_fc3d_f489,
                0x3706_959b_d953_1b1b,
                0x190c_c9bc_bd71_821b,
                0xb465_3ed0_74a4_17c1,
                0xa3b4_a702_1294_bd5f,
                0x4609_39b5_d858_843b,
                0x6212_539a_1b99_b8fc,
                0x169d_7f49_d432_0f3e,
                0x16a8_14b2_7e75_3973,
                0xd9f9_a97e_c370_47b2,
                0x14bf_dbb4_4bcf_decd,
                0x5bbd_4a18_2eb3_0789,
                0xeefa_81b5_904f_2225,
                0xd00c_40a0_70cb_6987,
                0x0788_cabd_62d5_8a6b,
                0xf4b2_e594_9aa2_f602,
                0x8eb1_0c9c_6790_a61f,
                0xc5b7_e4a6_030e_0af8,
                0xe30f_220b_d387_79d0,
                0x329f_bdb1_1aa6_5c9e,
                0xb6c3_425f_3ed3_6fbb,
                0xd9d5_9065_15c6_534c,
                0x7d7c_e0dd_7cd1_163f,
                0xbf82_da9e_4568_38a5,
                0xe4c8_8efb_42a8_98ce,
                0xad7a_5051_5928_edab,
                0x50f7_ed0a_3ad0_f802,
                0x6516_a296_941b_24e7,
                0x146b_0b25_650d_75f3,
                0x7cb8_27ea_c9b9_959c,
                0xcaf2_1d90_4c8d_2e89,
                0xd62c_2040_b9bc_ef06,
                0xe7bb_1076_dd2a_481a,
                0x0b79_11da_50a3_0902,
                0x91d4_bdd9_f01f_b290,
                0xd86b_cda4_cacf_6d2d,
                0xb10b_861d_1284_1187,
                0xa292_aec1_690d_b4fd,
                0xfa59_2676_5d8c_0215,
                0x199c_24ac_958f_a323,
                0x774e_12a3_d524_d466,
                0x2660_af82_4c16_2686,
                0xf46e_050a_c6a5_2c42,
                0x25cc_d03c_8882_dac1,
                0x3246_ccbb_b6a0_7448,
                0x594d_db7d_946e_70ff,
                0x0e5a_2c5c_8a5c_edc5,
                0xba3c_7cab_0f43_8558,
                0x341f_63ab_8d2b_cccd,
                0x5a4b_5fd0_a38a_22f3,
                0x81ee_99a1_f201_7cb2,
                0xfb01_ca01_22f1_ff08,
                0xd62f_21fa_fd00_55e2,
                0x5d9a_6296_9265_e330,
                0xe203_c3c6_f3fc_85e2,
                0xcc43_3404_5bce_1cf5,
                0x7752_e62a_752e_e7e7,
                0x83ee_3ea2_d042_1a13,
                0x9cf1_24b7_889d_2509,
                0xd0b8_b3be_c58d_fbcc,
                0x5240_bbb3_7ca0_a69e,
                0x47c6_2b4a_0c45_d6e5,
                0x556c_2aa7_f73a_3bc4,
                0x6cc2_5a62_8e46_55a8,
                0x775e_3209_c602_3728,
                0x3b10_d8ef_ff0a_b325,
                0x2480_55de_aeea_d73b,
                0xb48e_277d_bcd4_14b4,
                0x5f01_f0b2_5dc9_47fc,
                0x8e6c_6293_04e0_09f9,
                0x14d6_03f2_5282_eaa9,
                0x070e_fc2b_9482_2ddb,
                0x2bec_c6ec_2f43_1752,
                0x84be_d6db_d09a_0c74,
                0x80d1_25f7_1627_4561,
                0xd6f6_7d0c_efb0_fe54,
                0xcabc_fcc9_0d2c_acc3,
                0x00e1_a4b7_83a5_0861,
                0x7b66_6fd2_a17e_e5e6,
                0x27d6_1cad_5a1f_df57,
                0x3ace_8e1a_8d7c_c41e,
                0x48bc_352d_7edb_a68f,
                0x17ea_e678_a6b6_4b33,
                0x45a7_3f21_d058_ffe2,
                0x1731_1285_b8a4_e67e,
                0xf4ab_e9ac_8a25_528e,
                0xf618_bc91_e317_b657,
                0x56a8_2771_6ddc_63bb,
                0xde22_ea42_d310_e430,
                0x10d8_0acf_40f5_5beb,
                0x9740_c22e_8a39_9070,
                0x96b1_febd_6d54_fe26,
                0x7ae2_ab03_9945_5d07,
                0x3a49_7989_cb65_ad59,
                0x2c8f_cdf4_9a4a_4802,
                0x4fb3_49bf_8f03_9351,
                0x31ce_ad56_7979_590d,
                0xf616_32fe_71d2_71a7,
                0x1732_570d_9bc5_13a4,
                0xe286_8171_aeb5_1086,
                0x7db5_1cad_9ab9_7544,
                0x0626_ff61_f270_dc74,
                0x36c8_fbe6_1249_8248,
                0x30b5_2980_7a4c_40e6,
                0x8a7e_ba90_1101_1d50,
                0x4dec_1dcf_61ba_5fe6,
                0x15ba_d2c2_1d1c_808d,
                0x4c2c_f7d9_501f_0075,
                0x05f7_4859_7298_7e32,
                0xb18c_961c_70dd_8f1a,
                0xc3dd_5f34_f853_4336,
                0xd9ce_205b_05d7_e744,
                0x660a_9ba5_6437_fbae,
                0x5732_6b54_d3ff_bc43,
                0x1e9f_27aa_48a3_d948,
                0xdd39_2d38_1894_3f19,
                0x4952_6ccc_5333_30c9,
                0x6c5d_56b9_25bd_2486,
                0x1c01_e188_6c8d_4293,
                0x5404_b40c_bf6e_ae13,
                0x8718_a27e_3425_f68e,
                0xebc4_5895_b8e2_5f21,
                0x827e_ad30_59a5_36c6,
                0x53f7_8896_f903_7de9,
                0x3055_6231_8011_0138,
                0xe905_d491_9999_f4a8,
                0x41cf_2d3c_99fd_fffc,
                0x7aaf_f518_e73e_21d4,
                0xcf6e_582b_a118_f64b,
                0x5d82_dc70_e17a_de57,
                0xa6e4_309f_4795_3178,
                0x9de0_af8f_55fc_f890,
                0x405b_5798_08ef_0229,
                0x767e_f23a_83fb_4386,
                0xf7f1_adc9_ae89_9862,
                0x59aa_96e9_8aae_4fde,
                0x5ce9_16f6_b253_8d9e,
                0xa5cc_fb6e_fdb5_dc69,
                0x2ee1_4981_bada_0b73,
                0xb444_b130_cc93_e0dd,
                0x6071_eab7_5971_ca43,
                0x83a4_fc17_9b26_9eb4,
                0x2356_8d90_8e45_d0a0,
                0xd166_e21b_b649_3fde,
                0xad83_7631_fa7d_125a,
                0xdd5d_a16f_1fcf_925a,
                0xea20_0ba9_1e9f_db90,
                0x9dc0_52cd_9432_65fd,
                0xae30_e3f2_26a1_244b,
                0xb4e8_2545_574c_a2fb,
                0x2918_a82e_25a7_f0f5,
                0x0cbf_0bb7_088d_19c1,
                0x6b5a_4143_dfa2_8689,
                0xb36d_ee4c_b22f_9aa1,
                0x996d_f6f7_d530_ec53,
                0x3bda_9723_e487_9231,
                0x5e75_6d24_db8d_1bbe,
                0xa9c0_512d_3bb9_301a,
                0xb480_a1fa_7095_7fe3,
                0x0743_2ff3_88db_7bfc,
                0x0f8a_11eb_8ff6_4a7d,
                0x634a_a33b_746f_914c,
                0xa965_2209_5e29_62d2,
                0x1651_98cf_7014_84e7,
                0x1039_7336_48e5_445d,
                0x872d_b5ee_b165_ab3f,
                0x1556_fbd0_8e8f_9063,
                0x7048_f084_3d04_5dc8,
                0x4b33_fe5f_5f48_2d12,
                0x036e_0cf5_5754_0689,
                0xa11f_0cf7_1a17_4c52,
                0xaca9_1caf_9613_2468,
                0x393e_8207_96db_0bc5,
                0x2998_6d29_60b0_a0b1,
                0x2644_a383_9804_26fb,
                0xc396_d624_5ebd_901c,
                0x1307_07c7_0476_e11e,
                0x057e_14c8_c64f_2467,
                0xb46c_7432_ba47_9715,
                0x6f25_ac38_1e16_de0f,
                0x0fcd_b462_b762_ff43,
                0x1a70_0fd3_3bb6_42e3,
                0x6d70_5677_4ea7_d30c,
                0xd751_4dce_55d7_e751,
                0xf708_ec6f_0946_6361,
                0xbcb5_3da5_37d0_0a26,
                0xc82a_ed65_059c_bee7,
                0x58df_c924_a78c_b2f4,
                0x7c99_530e_9884_0c74,
                0x43b5_525b_e7d1_9215,
                0xea2e_5ab5_20e0_0f04,
                0x1894_c25a_8e95_06e3,
                0xf6ca_ec7e_ee5e_391a,
                0xfd06_9f11_396d_19cc,
                0x08da_5b27_8ee5_c808,
                0x0d5e_aab9_dc7c_c5dd,
                0xd73c_a0c1_6674_6172,
                0x3c3e_c131_dedb_fa34,
                0xcdab_d9c1_52c4_4e97,
                0xab93_efbb_456d_8c8c,
                0x8972_cabd_807d_bbf0,
                0xf783_a0d8_ba03_b38f,
                0xb88f_7e5b_49fa_7a14,
                0x2d1b_1909_44f6_475e,
                0x3488_6bdb_7ea7_bd9e,
                0x855c_ff9c_b31d_9ccb,
                0xdd8e_5a21_5714_a6d5,
                0xe7de_3a03_e7e4_1f1c,
                0x9828_a96c_c87b_afe6,
                0x16f6_3627_517e_f2e2,
                0x2011_fc88_f692_f494,
                0x8faf_dd09_d697_71a7,
                0x1b62_074f_bd5e_d534,
                0x8f9c_4d96_fee2_083d,
                0xa898_7ad1_74d3_8110,
                0x4365_00c3_6333_c8bb,
                0x447e_f308_19f7_4d94,
                0x972e_f7c6_7e0b_4dd0,
                0xc0de_e4db_1356_34ed,
                0xb67a_ebe9_e05d_da48,
                0x6ee7_07dc_c86c_6dad,
                0xdfcd_3bb9_7683_15d5,
                0x63d9_1a04_0e39_ff32,
                0xd80d_6668_5f52_94fc,
                0x973e_6fb5_87b3_9acf,
                0x80cf_42a8_ef53_4e2f,
                0xb881_ce3a_db07_116f,
                0xfcaa_a5d5_2e53_4257,
                0xe891_0f1f_6db0_1b00,
                0x1db5_e501_9f44_82f2,
                0x6457_c348_7936_30e4,
                0x8699_ca7a_3579_4698,
                0xc337_aad0_d508_a19b,
                0xfce7_baa3_393c_e700,
                0x847e_5f65_1a3e_7833,
                0xf939_ec01_fe97_06e5,
                0x21c8_4e7b_0bfe_f8ce,
                0xf70b_bbc0_eec7_1fd3,
                0xd706_ecad_2747_2f0a,
                0x4245_7f9c_f318_9daf,
                0x6d7d_4c68_db65_b44a,
                0xc818_3887_6add_e9fa,
                0x9dd7_69e1_6a18_be16,
                0x0625_6902_d045_be98,
                0x9211_d931_a0fa_23a3,
                0x1f3c_f12b_3f81_c1c2,
                0x55e2_e771_79b6_a5e1,
                0x1074_e168_b4d5_cc29,
                0xa0b2_4a00_57db_1eca,
                0x2a84_3087_529e_5a05,
                0x4f16_8e38_473c_b7c6,
                0x185e_f9fa_36ad_eb7c,
                0x0724_3d4d_a255_8d8d,
                0x0b70_6e52_151f_7ff5,
                0xf4ab_f0ed_705e_e7fb,
                0x68a0_9cd5_87b6_04fc,
                0x5dca_2d1a_8db6_1024,
                0x8107_4a3b_8539_419b,
                0xf964_dbfb_c808_e810,
                0x47b2_ed45_5e01_cddd,
                0xc5e4_2f2e_b982_2d95,
                0xdf34_db47_9f2d_7795,
                0x3921_324f_42b9_5d8c,
                0xc46f_f527_fada_0d81,
                0x7acc_69bc_310b_3661,
                0xf2e6_444c_eb50_2c94,
                0x8e5f_edf6_eb5a_c072,
                0xa420_14a7_7975_b648,
                0xfa32_bf5e_1cc3_8cc7,
                0x6653_77ee_f12c_e964,
                0xca39_9705_66fe_b4b7,
                0x7a92_7a84_d837_7277,
                0x5900_68a2_f63e_293d,
                0x715a_3077_1e19_1a6e,
                0x4c7b_0b65_910b_916c,
                0xdd2a_d2c9_91b9_537a,
                0x26da_2d1a_d999_af64,
                0xe8fe_1ed9_ab99_6c37,
                0x294c_da4b_6520_d1bc,
                0xdfd7_5dd4_7748_cebc,
                0xd681_e6b5_44dd_e3cb,
                0x7741_8f43_0321_c840,
                0xc509_a278_16b3_6a00,
                0x0b15_0510_3032_d5b9,
                0x27e9_01c8_9d4b_c50c,
                0x7852_65cc_f536_e590,
                0xd4eb_9da2_0693_1b4e,
                0x6bf9_25db_b2eb_1b49,
                0x72bd_11fc_6377_fd1f,
                0x49c2_d7b9_59c0_5b94,
                0xd2a1_aeac_7b28_a213,
                0xf22f_8d48_0cb8_b646,
                0x487a_dc56_e61d_ee09,
                0x2340_039e_813e_646e,
                0x54bc_259e_c076_e671,
                0x6efe_f4ea_800c_334a,
                0xa564_36a9_33b9_282c,
                0xc57c_b080_3ee3_58b9,
                0x04dc_102d_a09e_f993,
                0x7fd5_1ad2_19b3_2e87,
                0x106f_ea4b_3cf3_49fe,
                0x24b6_e7dc_7af7_72ba,
                0xcf8a_f004_5ed6_cc16,
                0x859a_c325_7196_c7fa,
                0xee20_756e_71d9_10e0,
                0x910b_faa7_74e6_5334,
                0xd494_f202_f5de_381e, // highest
            ]))
        );
    }
}
