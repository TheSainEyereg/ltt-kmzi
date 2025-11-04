#[cfg(feature = "sbox_runtime")]
mod sbox_runtime;

mod sbox_pregen;

struct AES;
impl AES {
    pub fn new() -> AES {
        AES {}
    }
}

impl Default for AES {
    fn default() -> Self {
        AES::new()
    }
}
