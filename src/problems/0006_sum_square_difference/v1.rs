//
// Sum square difference, v1
// https://projecteuler.net/problem=6
//
// The sum of the squares of the first ten natural numbers is,
//     1^2 + 2^2 + ... + 10^2 = 385
// The square of the sum of the first ten natural numbers is,
//     (1 + 2 + ... + 10)^2 = 55^2 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and
// the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural
// numbers and the square of the sum.
//

pub fn run() {
    let n = 100;

    let mut sum_of_the_squares = 0;
    for i in 1..=n {
        sum_of_the_squares += i * i;
    }

    let sum = n * (n + 1) / 2;

    println!("Square sum difference: {}", sum * sum - sum_of_the_squares);
}
