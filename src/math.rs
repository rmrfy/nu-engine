pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn subtract_arrays(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for a in arr1 {
        for &b in &arr2 {
            if a != b {
                result.push(a - b);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::subtract_arrays;

    #[test]
    fn subtracts_each_unequal_pair_in_order() {
        assert_eq!(
            subtract_arrays(vec![1, 2, 2], vec![2, 3]),
            vec![-1, -2, -1, -1]
        );
    }

    #[test]
    fn empty_arrays_produce_no_differences() {
        assert!(subtract_arrays(vec![], vec![1]).is_empty());
        assert!(subtract_arrays(vec![1], vec![]).is_empty());
        assert!(subtract_arrays(vec![], vec![]).is_empty());
    }

    #[test]
    fn equal_pairs_produce_no_differences() {
        assert!(subtract_arrays(vec![2, 2], vec![2]).is_empty());
    }
}
