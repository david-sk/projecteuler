//
// Strong Repunits, v1
// https://projecteuler.net/problem=346
//
// The number 7 is special, because 7 is 111 written in base 2, and 11 written in base 6
// (i.e. 7_10 = 11_6 = 111_2). In other words, 7 is a repunit in at least two bases b > 1.
// We shall call a positive integer with this property a strong repunit. It can be verified that
// there are 8 strong repunits below 50: {1,7,13,15,21,31,40,43}.
// Furthermore, the sum of all strong repunits below 1000 equals 15864.
// Find the sum of all strong repunits below 10^12.
//

//
// Notes:
// With `b` a base and `n` some number, we have: b*b*...*b + ... + b*b*b + b*b + b + 1 = n.
// (1) As we know that at least the case b + 1 = n can work for any number,
// the goal is to find a base `b` for the case b*b*...*b + ... + b*b*b + b*b = n - 1 - b.
// As the simplest case is b*b = n - 1 - b, we know that b*b <= n - 1 - b, so b*b <= n - 1,
// (2) and thus: b <= sqrt(n - 1).
// Thanks to (2), we go through 2 to sqrt(limit - 1) in the solution, that is, the max_base.
// And thanks to (1), we start checking numbers from 1 + b + b*b, that is, first value of `n`.
//

use std::collections::HashSet;

pub fn run() {
    let limit: u64 = 1000000000000;
    let max_base: u64 = ((limit - 1) as f64).sqrt() as u64;

    let mut found_numbers: HashSet<u64> = HashSet::new();
    let mut sum: u64 = 1; // 1 is of course valid so we include it directly in the sum

    for b in 2..=max_base {
        let mut base_powers: u64 = b * b;
        let mut n: u64 = 1 + b + base_powers;
        while n < limit {
            if !found_numbers.contains(&n) {
                sum += n;
                found_numbers.insert(n);
            }
            base_powers *= b;
            n += base_powers;
        }
    }

    println!("Strong repunits sum: {}", sum);
}
