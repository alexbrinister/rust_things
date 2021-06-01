fn main() {
    // Hardcode for now...
    let n : i64 = 5;
    let answer = factorial(n);

    println!("Factorial of {}:\t{}", n, answer);
}

// Naive factorial method using recursion
fn factorial(n: i64) -> i64 {
    if n == 1 {
        return 1;
    }

    return n * factorial(n - 1);
}
