pub fn map_values(
    value: f32,
    input_min: f32,
    input_max: f32,
    target_min: f32,
    target_max: f32,
) -> f32 {
    let slope = (target_max - target_min) as f32 / (input_max - input_min) as f32;
    let value = (value - input_min) as f32 * slope + target_min as f32;

    // handle floating point bugs
    // handle over max case
    let max_checked_value = f32::min(value, target_max);

    // handle under min case
    let max_min_checked_value = f32::max(max_checked_value, target_min);

    max_min_checked_value
}

#[cfg(test)]
mod tests {
    use crate::map_values::map_values;

    #[test]
    fn test_map_values_clean_up() {
        let test_value = map_values(3.0, 0.0, 10.0, 0.0, 100.0);
        assert_eq!(30 as f32, test_value);
    }

    #[test]
    fn test_map_values_clean_down() {
        let test_value = map_values(5.0, 0.0, 100.0, 0.0, 20.0);
        assert_eq!(1 as f32, test_value);
    }

    #[test]
    fn test_map_values_up() {
        let test_value = map_values(3.0, 0.0, 10.0, 0.0, 25.0);
        assert_eq!(7.5, test_value);
    }

    #[test]
    fn test_map_values_down() {
        let test_value = map_values(5.0, 0.0, 10.0, 0.0, 5.0);
        assert_eq!(2.5, test_value);
    }
}
