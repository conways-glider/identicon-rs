use sha2::{
    Sha512,
    Sha512Trunc256,
    Digest
};

pub fn generate_full_grid(image_size: usize, input_value: &str) -> Vec<bool> {
    // Compute the hash value
    let hash = Sha512::digest(input_value.as_bytes());
    // let hash_slice = hash.as_slice();
    let square_count = image_size.pow(2);
    let grid: Vec<bool> = (0..square_count)
        .map(
            |location|
            {
                println!("{:?}", location);
                hash[location] % 2 == 0
            }
        )
        .collect();
    grid
}
