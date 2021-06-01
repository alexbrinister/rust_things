fn main() {
    // Hardcode for now
    let a: [i32; 5] = [5, 6, 7, 8, 9];
    let answer = count_nums(&a);

    println!("Array a, with {} numbers, has {} elements", a.len(), answer);
}

fn count_nums(arr: &[i32]) -> i32 {
    // Base case: last item in array
    if arr.len() == 1 {
        return 1;
    }

    let rest_of_list = &arr[1..];

    return 1 + count_nums(rest_of_list);
}
