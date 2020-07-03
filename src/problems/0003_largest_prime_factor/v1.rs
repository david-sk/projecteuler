//
// Largest prime factor, v1
// https://projecteuler.net/problem=3
//
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
//

mod prime;

use prime::is_prime;

pub fn run() {
    let n = 600851475143;
    let mut largest_prime_factor = -1;

    if is_prime(n) {
        largest_prime_factor = n
    } else {
        let mut divided_n = n;
        let mut i = 2;
        while i < n {
            if n % i == 0 && is_prime(i) {
                largest_prime_factor = i;
                while divided_n % i == 0 {
                    divided_n /= i;
                }
                if divided_n == 1 {
                    break;
                }
            }
            i = if i == 2 { 3 } else { i + 2 }
        }
    }

    if largest_prime_factor == -1 {
        println!("No largest prime factor found")
    } else {
        println!("Largest prime factor: {}", largest_prime_factor);
    }
}
