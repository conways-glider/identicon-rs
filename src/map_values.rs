pub fn map_values(
    value: u32,
    input_min: u32,
    input_max: u32,
    target_min: u32,
    target_max: u32,
) -> f32 {
    let slope = (target_max - target_min) as f32 / (input_max - input_min) as f32;
    let mapped_value = target_min as f32 + slope * (value - input_min) as f32;
    mapped_value
}
