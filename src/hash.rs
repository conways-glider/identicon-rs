use sha3::{Digest, Sha3_256};

pub fn hash_value(input_value: &str) -> Vec<u8> {
    let input_trimmed = input_value.trim();
    Sha3_256::digest(input_trimmed.as_bytes())
        .as_slice()
        .to_vec()
}
