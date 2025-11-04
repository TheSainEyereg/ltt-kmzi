use super::gf256::GF;

const SBOX_MATRIX: [[u8; 8]; 8] = [
    [1, 0, 0, 0, 1, 1, 1, 1],
    [1, 1, 0, 0, 0, 1, 1, 1],
    [1, 1, 1, 0, 0, 0, 1, 1],
    [1, 1, 1, 1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 0, 0, 0],
    [0, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 0],
    [0, 0, 0, 1, 1, 1, 1, 1],
];

const SBOX_VECTOR: [u8; 8] = [1, 1, 0, 0, 0, 1, 1, 0];

pub fn generate_sbox_value(value: u8) -> u8 {
    let mut inverse = 0;

    if value != 0 {
        inverse = GF::inverse(value);
    }

    let mut result: u8 = 0;

    for bit in 0..8 {
        let current_bit = ((SBOX_MATRIX[bit][0] * (inverse & 0b00000001))
            ^ (SBOX_MATRIX[bit][1] * ((inverse & 0b00000010) >> 1))
            ^ (SBOX_MATRIX[bit][2] * ((inverse & 0b00000100) >> 2))
            ^ (SBOX_MATRIX[bit][3] * ((inverse & 0b00001000) >> 3))
            ^ (SBOX_MATRIX[bit][4] * ((inverse & 0b00010000) >> 4))
            ^ (SBOX_MATRIX[bit][5] * ((inverse & 0b00100000) >> 5))
            ^ (SBOX_MATRIX[bit][6] * ((inverse & 0b01000000) >> 6))
            ^ (SBOX_MATRIX[bit][7] * ((inverse & 0b10000000) >> 7)))
            ^ SBOX_VECTOR[bit];

        result |= current_bit << bit;
    }

    result
}

pub fn pregenerate_sbox() -> Vec<u8> {
    let mut sbox = vec![0; 256];

    for i in 0..256 {
        sbox[i] = generate_sbox_value(i as u8);
    }

    sbox
}

pub fn pregenerate_sbox_inv() -> Vec<u8> {
    let mut sbox_inv = vec![0; 256];

    for i in 0..256 {
        sbox_inv[generate_sbox_value(i as u8) as usize] = i as u8;
    }

    sbox_inv
}

#[cfg(test)]
mod tests {
    use super::generate_sbox_value;
    use crate::crypto::aes::sbox_pregen::{SBOX, SBOX_INV};

    #[test]
    fn sbox() {
        let byte1 = 0x00u8;
        let byte2 = 0xffu8;

        assert_eq!(SBOX[byte1 as usize], generate_sbox_value(byte1));
        assert_eq!(SBOX[byte2 as usize], generate_sbox_value(byte2));
    }

    #[test]
    fn sbox_inv() {
        let byte1 = 0x52u8;
        let byte2 = 0xffu8;

        assert_eq!(byte1, SBOX_INV[generate_sbox_value(byte1) as usize]);
        assert_eq!(byte2, SBOX_INV[generate_sbox_value(byte2) as usize]);
    }

    #[test]
    fn sbox_generation() {
        assert_eq!(SBOX.to_vec(), super::pregenerate_sbox());
        assert_eq!(SBOX_INV.to_vec(), super::pregenerate_sbox_inv());
    }
}
