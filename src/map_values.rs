pub fn map_values(
    value: u32,
    input_min: u32,
    input_max: u32,
    target_min: u32,
    target_max: u32,
) -> f32 {
    let slope = (target_max - target_min) as f32 / (input_max - input_min) as f32;
    let mapped_value = (value - input_min) as f32 * slope + target_min as f32;
    mapped_value
}

#[cfg(test)]
mod tests {
    use crate::map_values::map_values;

    #[test]
    fn test_map_values_clean_up() {
        let test_value = map_values(3, 0, 10, 0, 100);
        assert_eq!(30 as f32, test_value);
    }

    #[test]
    fn test_map_values_clean_down() {
        let test_value = map_values(5, 0, 100, 0, 20);
        assert_eq!(1 as f32, test_value);
    }

    #[test]
    fn test_map_values_up() {
        let test_value = map_values(3, 0, 10, 0, 25);
        assert_eq!(7.5, test_value);
    }

    #[test]
    fn test_map_values_down() {
        let test_value = map_values(5, 0, 10, 0, 5);
        assert_eq!(2.5, test_value);
    }
}
