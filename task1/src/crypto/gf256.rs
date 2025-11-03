const MOD: u8 = 0b00011011; //x^8 + x^4 + x^3 + x^1 + 1 => 100011011 as u8 => 00011011

pub fn add(a: u8, b: u8) -> u8 {
    a ^ b
}

pub fn mul(mut a: u8, mut b: u8) -> u8 {
    let mut res: u8 = 0;

    while b != 0 {
        if b & 1 != 0 {
            res ^= a;
        }

        let c = a & 0x80;
        a <<= 1;

        if c != 0 {
            a ^= MOD;
        }

        b >>= 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::crypto::gf256::mul;

    #[test]
    fn gf_basics() {
        assert_eq!(0b00011011, super::MOD);
        assert_eq!(0b00011010, mul(0x55, 0x5))
    }
}
