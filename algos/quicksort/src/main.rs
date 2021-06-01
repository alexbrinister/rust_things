use rand::prelude::*;

fn main() {
    let vec: Vec<i32> = vec![32, 11, 4, 79, 101, 2, 1, 99, 3, 20];
    println!("Original Array: {:?}", vec);

    let sorted_vec = quicksort(vec);
    println!("Sorted array: {:?}", sorted_vec);
}

// Quicksort for arrays of integers
// Input: an integer array slice
// Output: a sorted version of the input slice
fn quicksort(vec: Vec<i32>) -> Vec<i32> {
    // Base case: an array of 0 or 1 elements
    if vec.len() < 2 {
        return vec;
    }

    // Recursive case
    let mut rng = thread_rng();
    let pivot_index = rng.gen_range(0..vec.len());
    let pivot = vec[pivot_index];

    let mut less : Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        if vec[i] < pivot {
            less.push(vec[i])
        }
    }

    let mut greater : Vec<i32> = Vec::new();
    for i in 0..vec.len() {
        if vec[i] > pivot {
            greater.push(vec[i])
        }
    }
    
    let mut ret : Vec<i32> = Vec::new();
    ret.extend(quicksort(less));
    ret.push(pivot);
    ret.extend(quicksort(greater));

    return ret;
}
