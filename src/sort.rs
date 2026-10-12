pub fn ascending(values: &mut [i32]) {
    values.sort_unstable();
}

pub fn descending(values: &mut [i32]) {
    values.sort_unstable_by(|a, b| b.cmp(a));
}

/// Stable ascending merge sort with O(n log n) time and O(n) extra space.
pub fn merge_sort(values: &mut [i32]) {
    if values.len() < 2 {
        return;
    }
    let mut scratch = vec![0; values.len()];
    merge_sort_with_scratch(values, &mut scratch);
}

fn merge_sort_with_scratch(values: &mut [i32], scratch: &mut [i32]) {
    if values.len() < 2 {
        return;
    }
    let middle = values.len() / 2;
    let (left, right) = values.split_at_mut(middle);
    let (left_scratch, right_scratch) = scratch.split_at_mut(middle);
    merge_sort_with_scratch(left, left_scratch);
    merge_sort_with_scratch(right, right_scratch);

    let (mut i, mut j) = (0, 0);
    for output in scratch.iter_mut() {
        if j == right.len() || (i < left.len() && left[i] <= right[j]) {
            *output = left[i];
            i += 1;
        } else {
            *output = right[j];
            j += 1;
        }
    }
    values.copy_from_slice(scratch);
}

/// Ascending heap sort with O(n log n) time and O(1) extra space.
pub fn heap_sort(values: &mut [i32]) {
    for root in (0..values.len() / 2).rev() {
        sift_down(values, root);
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        sift_down(&mut values[..end], 0);
    }
}

fn sift_down(values: &mut [i32], mut root: usize) {
    while root < values.len() / 2 {
        let mut child = root * 2 + 1;
        if child + 1 < values.len() && values[child] < values[child + 1] {
            child += 1;
        }
        if values[root] >= values[child] {
            break;
        }
        values.swap(root, child);
        root = child;
    }
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

    #[test]
    fn advanced_sorts_match_standard_sort() {
        let cases = [
            vec![],
            vec![42],
            vec![2, 1],
            vec![3, i32::MIN, 0, 3, -1, i32::MAX],
            vec![7; 17],
            (-50..=50).collect(),
            (-50..=50).rev().collect(),
        ];
        for sort in [merge_sort, heap_sort] {
            for input in &cases {
                let mut expected = input.clone();
                expected.sort();
                let mut actual = input.clone();
                sort(&mut actual);
                assert_eq!(actual, expected);
            }
            // Exercise uneven partitions and heaps with one or two children.
            for length in 0..128 {
                let mut actual: Vec<i32> = (0..length)
                    .map(|i| (i * 37 + length * 13) % 23 - 11)
                    .collect();
                let mut expected = actual.clone();
                expected.sort();
                sort(&mut actual);
                assert_eq!(actual, expected);
            }
        }
    }
}
