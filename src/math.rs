pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn bad_sum(arr: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..arr.len() {
        total += arr[i];
    }
    total
}
