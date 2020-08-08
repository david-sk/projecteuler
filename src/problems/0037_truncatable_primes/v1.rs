//
// Truncatable primes, v1
// https://projecteuler.net/problem=37
//
// The number 3797 has an interesting property. Being prime itself, it is possible to continuously
// remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7.
// Similarly we can work from right to left: 3797, 379, 37, and 3.
// Find the sum of the only eleven primes that are both truncatable from left to right and
// right to left.
// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
//

mod prime;

use prime::is_prime;

fn are_remaining_digits_prime(n: u64) -> bool {
    let mut a = n / 10;
    let mut ten_powers = 1;
    while a > 0 {
        if !is_prime(a) {
            return false;
        }
        a /= 10;
        ten_powers *= 10;
    }
    let mut b = n - n / ten_powers * ten_powers;
    while b > 0 {
        if !is_prime(b) {
            return false;
        }
        ten_powers /= 10;
        b -= b / ten_powers * ten_powers;
    }
    return true;
}

pub fn run() {
    let limit = 11;
    let mut num_found_truncatable_prime = 0;
    let mut sum = 0;
    let mut n = 11;
    while num_found_truncatable_prime < limit {
        if is_prime(n) && are_remaining_digits_prime(n) {
            num_found_truncatable_prime += 1;
            sum += n;
        }
        n += 2;
    }
    println!("Sum: {}", sum);
}
