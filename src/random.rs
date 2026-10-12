use rand::Rng;

/// Returns a random integer in the inclusive range `min..=max`.
///
/// # Panics
///
/// Panics if `min` is greater than `max`.
pub fn random_int(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

/// Returns a random floating-point number in the range `0.0..1.0`.
pub fn random_float() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers_stay_in_range() {
        for _ in 0..1_000 {
            assert!((-10..=10).contains(&random_int(-10, 10)));
        }
    }

    #[test]
    fn single_value_range_returns_that_value() {
        assert_eq!(random_int(7, 7), 7);
    }

    #[test]
    fn full_integer_range_is_supported() {
        let _ = random_int(i32::MIN, i32::MAX);
    }

    #[test]
    #[should_panic]
    fn reversed_range_panics() {
        random_int(1, 0);
    }

    #[test]
    fn floats_stay_in_range() {
        for _ in 0..1_000 {
            assert!((0.0..1.0).contains(&random_float()));
        }
    }
}
