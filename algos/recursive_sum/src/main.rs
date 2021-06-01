fn main() {
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let recursive_answer = recursive_sum(&arr);

    // Naive check summation
    let mut naive_answer = 0;
    for n in arr.iter() {
        naive_answer += n;
    }

    println!("Naive answer: {}\tRecursive answer: {}", 
             naive_answer, 
             recursive_answer);
}

fn recursive_sum(arr: &[i32]) -> i32 {
    if arr.len() == 0 {
        return 0;
    }

    return &arr[0] + recursive_sum(&arr[1..]);
}
