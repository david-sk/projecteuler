//
// Multiples of 3 and 5, v1
// https://projecteuler.net/problem=1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
//

// NOTE: Here is a dummy version (we start at i = 0 instead of 1, which is useless)
// for the example of having multiple problem versions.
pub fn run() {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("Multiples of 3 or 5 sum: {}", sum);
}
