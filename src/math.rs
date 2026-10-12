pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a.wrapping_sub(b)
}

#[cfg(test)]
mod tests {
    use super::subtract;

    #[test]
    fn subtracts_without_overflow() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(3, 5), -2);
        assert_eq!(subtract(-5, -3), -2);
        assert_eq!(subtract(0, 0), 0);
    }

    #[test]
    fn subtraction_wraps_on_overflow() {
        assert_eq!(subtract(i32::MIN, 1), i32::MAX);
        assert_eq!(subtract(i32::MAX, -1), i32::MIN);
        assert_eq!(subtract(0, i32::MIN), i32::MIN);
    }
}
