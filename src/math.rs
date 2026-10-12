pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn sort_numbers(arr: &mut Vec<i32>) {
    arr.sort_unstable();
    arr.dedup();
}

#[cfg(test)]
mod tests {
    use super::sort_numbers;

    #[test]
    fn sorts_numbers_and_removes_duplicates() {
        let mut numbers = vec![3, -1, 2, 3, 0, -1, i32::MAX, i32::MIN];

        sort_numbers(&mut numbers);

        assert_eq!(numbers, vec![i32::MIN, -1, 0, 2, 3, i32::MAX]);
    }

    #[test]
    fn handles_empty_and_single_element_vectors() {
        let mut empty = vec![];
        sort_numbers(&mut empty);
        assert!(empty.is_empty());

        let mut single = vec![1];
        sort_numbers(&mut single);
        assert_eq!(single, vec![1]);
    }
}
