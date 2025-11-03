struct AES {}

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
