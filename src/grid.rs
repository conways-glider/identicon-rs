pub fn generate_full_grid<T: Into<u32>>(image_size: T, hash: &Vec<u8>) -> Vec<bool> {
    // Compute the hash value
    let square_count = (image_size.into() as usize).pow(2);
    let grid: Vec<bool> = (0..square_count)
        .map(|location| hash[location % hash.len()] % 2 == 0)
        .collect();
    grid
}
