#[cfg(feature = "sbox_runtime")]
mod sbox_runtime;

mod sbox_pregen;

pub struct AES;
impl AES {
    pub fn new() -> AES {
        AES {}
    }

    pub fn return_zero(&self) -> Vec<u8> {
        vec![0] // 🤡
    }
}

impl Default for AES {
    fn default() -> Self {
        AES::new()
    }
}
