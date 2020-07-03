//
// 10001st prime, v1
// https://projecteuler.net/problem=7
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
// we can see that the 6th prime is 13.
// What is the 10 001st prime number?
//

mod prime;

use prime::is_prime;

pub fn run() {
    // we assume n > 1 (strictly greater than 1)
    let n = 10001;
    let mut num_found_prime_numbers = 1;

    let mut number = 1;
    loop {
        number += 2;
        if is_prime(number) {
            num_found_prime_numbers += 1
        }
        if num_found_prime_numbers == n {
            break;
        }
    }

    println!("Prime number: {}", number);
}
