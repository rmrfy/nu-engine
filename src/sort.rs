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

pub fn insertion_sort(values: &mut [i32]) {
    for index in 1..values.len() {
        let value = values[index];
        let mut position = index;
        while position > 0 && values[position - 1] > value {
            values[position] = values[position - 1];
            position -= 1;
        }
        values[position] = value;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algorithms_sort_integer_arrays() {
        let algorithms: [fn(&mut [i32]); 3] = [bubble_sort, insertion_sort, selection_sort];
        let cases = [
            vec![],
            vec![1],
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
}
