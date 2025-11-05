#[cfg(feature = "sbox_runtime")]
mod sbox_runtime;

mod sbox_pregen;

pub enum Mode {
    AES128,
    AES192,
    AES256
}

pub struct AES {
    nk: u8,
    nb: u8,
    nr: u8
}
impl AES {
    pub fn new(mode: Mode) -> AES {
        let (nk, nb, nr) = match mode {
            Mode::AES128 => (4, 4, 10),
            Mode::AES192 => (6, 4, 12),
            Mode::AES256 => (8, 4, 14)
        };

        AES { nk, nb, nr }
    }

    pub fn return_zero(&self) -> Vec<u8> {
        vec![0] // 🤡
    }
}

impl Default for AES {
    fn default() -> Self {
        AES::new(Mode::AES128)
    }
}
