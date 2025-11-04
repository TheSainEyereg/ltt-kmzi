mod aes;
mod ofb;

pub fn encrypt() -> Vec<u8> {
    let aes = aes::AES::new();

    aes.return_zero()
}
