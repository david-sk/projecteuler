//
// Summation of primes, v1
// https://projecteuler.net/problem=10
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// Find the sum of all the primes below two million.
//

mod prime;

use prime::is_prime;

pub fn run() {
    let n = 2000000;
    let mut sum = if n < 2 { 0 } else { 2 };
    for i in (3..=n).step_by(2) {
        if is_prime(i) {
            sum += i;
        }
    }
    println!("Prime sum: {}", sum);
}
