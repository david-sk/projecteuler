//
// Prime permutations, v1
// https://projecteuler.net/problem=49
//
// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
// is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit
// numbers are permutations of one another.
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this
// property, but there is one other 4-digit increasing sequence.
// What 12-digit number do you form by concatenating the three terms in this sequence?
//

mod prime;

use prime::is_prime;
use std::collections::HashMap;

fn have_same_digits(mut a: u64, mut b: u64) -> bool {
    let mut a_digits: HashMap<u64, u64> = HashMap::new();
    while a > 0 {
        let a_mod_10 = a % 10;
        a_digits.insert(a_mod_10, a_digits.get(&a_mod_10).unwrap_or(&0) + 1);
        a /= 10;
    }
    while b > 0 {
        let b_mod_10 = b % 10;
        if !a_digits.contains_key(&b_mod_10) || a_digits[&b_mod_10] == 0 {
            return false;
        }
        a_digits.insert(b_mod_10, a_digits[&b_mod_10] - 1);
        b /= 10;
    }
    return true;
}

pub fn run() {
    let mut primes = vec![];
    for i in 1000..=9999 {
        if i != 1487 && i != 4817 && i != 8147 && is_prime(i) {
            primes.push(i as u64);
        }
    }

    let primes_length = primes.len();
    for i in 0..primes_length {
        for j in (i + 1)..primes_length {
            if !have_same_digits(primes[i], primes[j]) {
                continue;
            }
            for k in (j + 1)..primes_length {
                if have_same_digits(primes[j], primes[k])
                    && (primes[j] - primes[i]) == (primes[k] - primes[j])
                {
                    println!("Result: {}{}{}", primes[i], primes[j], primes[k]);
                    return;
                }
            }
        }
    }

    println!("No result found :(");
}
