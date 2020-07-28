//
// Prime generating integers, v1
// https://projecteuler.net/problem=357
//
// Consider the divisors of 30: 1,2,3,5,6,10,15,30.
// It can be seen that for every divisor d of 30, d+30/d is prime.
// Find the sum of all positive integers n not exceeding 100 000 000
// such that for every divisor d of n, d+n/d is prime.
//

mod prime;

use prime::is_prime;

fn check_divisors(n: i64) -> bool {
    let sqrt = (n as f64).sqrt() as i64;
    for d in 1..=sqrt {
        if n % d == 0 && !is_prime(d + n / d) {
            return false;
        }
    }
    return true;
}

pub fn run() {
    let limit = 100000000;
    let mut sum = 1; // 1 is valid so including it in the sum
    for i in (2..=limit).step_by(2) {
        if check_divisors(i) {
            sum += i;
        }
    }
    println!("Result: {}", sum);
}
