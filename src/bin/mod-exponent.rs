fn main() {
    println!("Modular exponentiation of 2^5 mod 17 is: {:?}", mod_exp(2, 5, 17));
}

fn mod_exp(base: u64, mut exp: u64, modulus: u64) -> Option<u64> {
    if modulus == 0 {
        return None;
    }

    let mut power: u64 = base % modulus;
    let mut result: u64 = 1;
    let mut tmp: u128;

    while exp > 0 {
        if exp & 0x01 == 0x01 {
            tmp = result as u128 * power as u128;
            result = (tmp % modulus as u128) as u64;
        }

        tmp = power as u128 * power as u128;
        power = (tmp % modulus as u128) as u64;

        exp >>= 1;
    }

    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_exp() {
        assert_eq!(mod_exp(2, 5, 17), Some(32 % 17));
        assert_eq!(mod_exp(3, 4, 7), Some(81 % 7));
        assert_eq!(mod_exp(5, 3, 13), Some(125 % 13));
        assert_eq!(mod_exp(10, 6, 11), Some(1000000 % 11));
        assert_eq!(mod_exp(7, 8, 19), Some(5764801 % 19));
    }

    #[test]
    fn test_mod_zero() {
        assert_eq!(mod_exp(0, 5, 17), Some(0));
        assert_eq!(mod_exp(2, 0, 17), Some(1));
        assert_eq!(mod_exp(0, 0, 17), Some(1));
    }

    #[test]
    fn test_mod_one() {
        assert_eq!(mod_exp(2, 5, 1), Some(0));
        assert_eq!(mod_exp(3, 4, 1), Some(0));
        assert_eq!(mod_exp(5, 3, 1), Some(0));
    }
}
