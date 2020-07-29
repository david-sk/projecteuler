//
// Consecutive positive divisors, v1
// https://projecteuler.net/problem=179
//
// Find the number of integers 1 < n < 10^7, for which n and n + 1 have the same number of positive
// divisors. For example, 14 has the positive divisors 1, 2, 7, 14 while 15 has 1, 3, 5, 15.
//

fn get_num_divisors(n: i64) -> i64 {
    let mut num_divisors = 0;
    let sqrt = (n as f64).sqrt();
    for i in 1..=(sqrt as i64) {
        if n % i == 0 {
            num_divisors += if (i as f64) == sqrt { 1 } else { 2 };
        }
    }
    return num_divisors;
}

pub fn run() {
    let limit = 10000000;
    let mut sum = 0;
    let mut prev_num_divisor = get_num_divisors(2);
    for i in 3..limit {
        let num_divisor = get_num_divisors(i);
        if num_divisor == prev_num_divisor {
            sum += 1;
        }
        prev_num_divisor = num_divisor;
    }
    println!("Result: {}", sum);
}
