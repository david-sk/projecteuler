//
// Quadratic primes, v1
// https://projecteuler.net/problem=27
//

mod prime;

use prime::is_prime;

pub fn run() {
    let limit: i64 = 999;
    let mut max_num_primes = 0;
    let mut ab_product = 0;
    for a in -limit..=limit {
        for b in -limit..=limit {
            let mut n = 0;
            while is_prime(n * n + a * n + b) {
                n += 1;
            }
            if n > max_num_primes {
                max_num_primes = n;
                ab_product = a * b;
            }
        }
    }
    println!("Result: {}", ab_product);
}
