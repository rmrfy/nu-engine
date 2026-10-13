pub fn ascending(values: &mut [i32]) {
    values.sort_unstable();
}

pub fn descending(values: &mut [i32]) {
    values.sort_unstable_by(|a, b| b.cmp(a));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_in_both_directions() {
        let mut values = [3, i32::MIN, 0, 3, -1, i32::MAX];
        ascending(&mut values);
        assert_eq!(values, [i32::MIN, -1, 0, 3, 3, i32::MAX]);
        descending(&mut values);
        assert_eq!(values, [i32::MAX, 3, 3, 0, -1, i32::MIN]);
    }

    #[test]
    fn handles_empty_and_single_element_slices() {
        let mut empty = [];
        ascending(&mut empty);
        descending(&mut empty);
        assert_eq!(empty, []);

        let mut single = [42];
        ascending(&mut single);
        descending(&mut single);
        assert_eq!(single, [42]);
    }
}
