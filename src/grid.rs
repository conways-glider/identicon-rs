use sha2::{Digest, Sha512};

pub fn generate_full_grid<T: Into<u32>>(image_size: T, input_value: &str) -> Vec<bool> {
    // Compute the hash value
    let hash = Sha512::digest(input_value.as_bytes());
    let square_count = (image_size.into() as usize).pow(2);
    let grid: Vec<bool> = (0..square_count)
        .map(|location| hash[location % hash.len()] % 2 == 0)
        .collect();
    grid
}
