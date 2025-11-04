const MOD: u8 = 0b00011011; //x^8 + x^4 + x^3 + x^1 + 1 => 100011011 as u8 => 00011011

pub struct GF;
impl GF {
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

    pub fn inverse(mut a: u8) -> u8 {
        let mut result = 1u8;
        let mut pow = u8::MAX - 1;

        while pow != 0 {
            if pow & 1 != 0 {
                result = Self::mul(result, a);
            }

            a = Self::mul(a, a);
            pow >>= 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::{GF, MOD};

    #[test]
    fn gf_basics() {
        assert_eq!(0b00011011, MOD);

        assert_eq!(0x55, GF::add(0x5, GF::add(0x5, 0x55)));
        assert_eq!(0b00011010, GF::mul(0x55, 0x5))
    }

    fn dumb_inverse(a: u8) -> u8 {
        for i in 0..255 {
            if GF::mul(a, i) == 1 {
                return i;
            }
        }

        0
    }

    #[test]
    fn gf_validate() {
        let b = 0xffu8;

        let dumb = dumb_inverse(b);
        let res = GF::inverse(b);

        assert_eq!(1, GF::mul(res, b));
        assert_eq!(dumb, res);
    }
}
