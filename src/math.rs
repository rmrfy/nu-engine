pub fn add(a: i32, b: i32) -> i32 {
    a.wrapping_add(b)
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b)
}

/// Returns `None` for division by zero; overflow wraps like Java integer division.
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a.wrapping_div(b))
    }
}

/// Returns `None` for division by zero; the remainder has the dividend's sign.
pub fn remainder(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a.wrapping_rem(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_arithmetic_wraps_like_java() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(subtract(2, 3), -1);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(add(i32::MAX, 1), i32::MIN);
        assert_eq!(subtract(i32::MIN, 1), i32::MAX);
        assert_eq!(multiply(i32::MAX, 2), -2);
    }

    #[test]
    fn division_truncates_toward_zero_and_handles_overflow() {
        assert_eq!(divide(7, 3), Some(2));
        assert_eq!(divide(-7, 3), Some(-2));
        assert_eq!(divide(7, -3), Some(-2));
        assert_eq!(divide(i32::MIN, -1), Some(i32::MIN));
        assert_eq!(divide(1, 0), None);
    }

    #[test]
    fn remainder_uses_the_dividend_sign_and_handles_overflow() {
        assert_eq!(remainder(7, 3), Some(1));
        assert_eq!(remainder(-7, 3), Some(-1));
        assert_eq!(remainder(7, -3), Some(1));
        assert_eq!(remainder(i32::MIN, -1), Some(0));
        assert_eq!(remainder(1, 0), None);
    }
}
