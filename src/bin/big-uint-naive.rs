use std::{
    cmp::{Ordering, max},
    ops::{Add, Mul, Sub},
};

fn main() {}

#[derive(Debug)]
struct BigNumber {
    data: Vec<u64>,
}

impl BigNumber {
    fn new(data: Vec<u64>) -> Self {
        BigNumber { data }
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

impl Mul for &BigNumber {
    type Output = BigNumber;

    fn mul(self, other: Self) -> BigNumber {
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
}
