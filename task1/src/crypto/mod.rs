mod aes;
mod ofb;

pub fn encrypt() -> Vec<u8> {
    let aes = aes::AES::default();

    aes.return_zero()
}
