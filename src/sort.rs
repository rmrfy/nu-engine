pub fn bubble_sort(values: &mut [i32]) {
    for end in (1..values.len()).rev() {
        let mut swapped = false;
        for index in 0..end {
            if values[index] > values[index + 1] {
                values.swap(index, index + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn selection_sort(values: &mut [i32]) {
    for index in 0..values.len() {
        let mut minimum = index;
        for candidate in index + 1..values.len() {
            if values[candidate] < values[minimum] {
                minimum = candidate;
            }
        }
        values.swap(index, minimum);
    }
}

pub fn merge_sort(values: &mut [i32]) {
    fn merge(values: &mut [i32], buffer: &mut [i32]) {
        if values.len() < 2 {
            return;
        }
        let middle = values.len() / 2;
        let (left, right) = values.split_at_mut(middle);
        let (left_buffer, right_buffer) = buffer.split_at_mut(middle);
        merge(left, left_buffer);
        merge(right, right_buffer);

        let (mut i, mut j) = (0, 0);
        for output in buffer.iter_mut() {
            if j == right.len() || (i < left.len() && left[i] <= right[j]) {
                *output = left[i];
                i += 1;
            } else {
                *output = right[j];
                j += 1;
            }
        }
        values.copy_from_slice(buffer);
    }

    let mut buffer = vec![0; values.len()];
    merge(values, &mut buffer);
}

pub fn quick_sort(mut values: &mut [i32]) {
    while values.len() > 1 {
        let pivot = values[values.len() / 2];
        let (mut lower, mut current, mut upper) = (0, 0, values.len());
        // Group equal values together so duplicates need no further sorting.
        while current < upper {
            if values[current] < pivot {
                values.swap(lower, current);
                lower += 1;
                current += 1;
            } else if values[current] > pivot {
                upper -= 1;
                values.swap(current, upper);
            } else {
                current += 1;
            }
        }
        let (left, rest) = values.split_at_mut(lower);
        let right = &mut rest[upper - lower..];
        // Recurse only on the smaller partition to bound stack depth.
        if left.len() < right.len() {
            quick_sort(left);
            values = right;
        } else {
            quick_sort(right);
            values = left;
        }
    }
}

pub fn heap_sort(values: &mut [i32]) {
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

    for root in (0..values.len() / 2).rev() {
        sift_down(values, root);
    }
    for end in (1..values.len()).rev() {
        values.swap(0, end);
        sift_down(&mut values[..end], 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algorithms_sort_integer_arrays() {
        let algorithms: [fn(&mut [i32]); 5] =
            [bubble_sort, selection_sort, merge_sort, quick_sort, heap_sort];
        let cases = [
            vec![],
            vec![1],
            vec![2, 1],
            vec![7; 32],
            vec![1, 2, 3, 4],
            vec![4, 3, 2, 1],
            vec![3, -1, 3, 0, -1],
            vec![i32::MAX, 0, i32::MIN],
        ];
        for algorithm in algorithms {
            for input in &cases {
                let mut actual = input.clone();
                let mut expected = input.clone();
                expected.sort();
                algorithm(&mut actual);
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn advanced_algorithms_sort_large_arrays() {
        for algorithm in [merge_sort, quick_sort, heap_sort] {
            for input in [
                (0..10_000).collect::<Vec<i32>>(),
                (0..10_000).rev().collect(),
                (0..10_000).map(|i| (i * 7919) % 101 - 50).collect(),
            ] {
                let mut expected = input.clone();
                expected.sort();
                let mut actual = input;
                algorithm(&mut actual);
                assert_eq!(actual, expected);
            }
        }
    }
}
