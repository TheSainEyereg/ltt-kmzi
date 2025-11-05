mod gf256;
use gf256::GF;

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

pub fn generate_sbox() -> Vec<u8> {
    let mut sbox = vec![0; 256];

    for i in 0..256 {
        sbox[i] = generate_sbox_value(i as u8);
    }

    sbox
}

pub fn generate_sbox_inv() -> Vec<u8> {
    let mut sbox_inv = vec![0; 256];

    for i in 0..256 {
        sbox_inv[generate_sbox_value(i as u8) as usize] = i as u8;
    }

    sbox_inv
}

pub fn generate_rcon(nr: u8) -> Vec<[u8; 4]> {
    let mut rc: Vec<u8> = vec![0; nr as usize];
    let mut out: Vec<[u8; 4]> = Vec::new();

    for round in 0..nr as usize {
        if round == 0 {
            rc[round] = 1;
        } else if rc[round - 1] < 0x80 {
            rc[round] = 2 * rc[round - 1];
        } else {
            rc[round] = (2 * rc[round - 1] as u16) as u8 ^ gf256::MOD;
        }
    }

    for round in 0..nr as usize {
        out.push([rc[round], 0, 0, 0]);
    }

    out
}

#[cfg(test)]
mod tests {
    use std::rc;

    use super::{generate_sbox, generate_sbox_inv, generate_rcon, generate_sbox_value};
    use crate::crypto::aes::const_pregen::{RCON, SBOX, SBOX_INV};

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
        assert_eq!(SBOX.to_vec(), generate_sbox());
        assert_eq!(SBOX_INV.to_vec(), generate_sbox_inv());
    }

    #[test]
    fn rcon_generation() {
        assert_eq!(RCON.to_vec(), generate_rcon(10))
    }
}
