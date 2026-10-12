pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Matches Java's integer absolute value: `i32::MIN` remains negative.
pub fn abs(a: i32) -> i32 {
    a.wrapping_abs()
}

pub fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

pub fn max(a: i32, b: i32) -> i32 {
    a.max(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication_handles_signs_and_zero() {
        assert_eq!(multiply(6, 7), 42);
        assert_eq!(multiply(-6, 7), -42);
        assert_eq!(multiply(-6, -7), 42);
        assert_eq!(multiply(i32::MAX, 0), 0);
    }

    #[test]
    fn absolute_value_matches_java_for_all_signs_and_minimum() {
        assert_eq!(abs(-42), 42);
        assert_eq!(abs(42), 42);
        assert_eq!(abs(0), 0);
        assert_eq!(abs(i32::MIN), i32::MIN);
        assert_eq!(abs(i32::MAX), i32::MAX);
    }

    #[test]
    fn min_and_max_handle_order_equality_and_extremes() {
        assert_eq!(min(-4, 7), -4);
        assert_eq!(min(7, -4), -4);
        assert_eq!(max(-4, 7), 7);
        assert_eq!(max(7, -4), 7);
        assert_eq!(min(3, 3), 3);
        assert_eq!(max(3, 3), 3);
        assert_eq!(min(i32::MIN, i32::MAX), i32::MIN);
        assert_eq!(max(i32::MIN, i32::MAX), i32::MAX);
    }
}
