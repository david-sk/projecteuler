//
// Circular primes, v1
// https://projecteuler.net/problem=35
//
// The number, 197, is called a circular prime because all rotations of the digits: 197, 971,
// and 719, are themselves prime.
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
// How many circular primes are there below one million?
//

mod prime;

use prime::is_prime;
use std::collections::HashSet;

fn get_digits(mut n: u64) -> Vec<u64> {
    let mut digits = vec![];
    while n > 0 {
        digits.insert(0, n % 10);
        n /= 10;
    }
    return digits;
}

fn get_digits_as_number(digits: &Vec<u64>, start: usize) -> u64 {
    let mut number = 0;
    let mut ten_powers = 1;
    let digits_length = digits.len();
    for i in start..digits_length {
        number += digits[digits_length - i - 1] * ten_powers;
        ten_powers *= 10;
    }
    for i in 0..start {
        number += digits[digits_length - i - 1] * ten_powers;
        ten_powers *= 10;
    }
    return number;
}

pub fn run() {
    let limit = 1000000;

    let mut found_circular_primes: HashSet<u64> = HashSet::new();
    found_circular_primes.insert(2); // include 2 to begin with

    for i in (3..limit).step_by(2) {
        if !is_prime(i) || found_circular_primes.contains(&i) {
            continue;
        }
        let mut found_primes = vec![i];
        let digits = get_digits(i);
        let mut ok = true;
        for start in 1..digits.len() {
            let number = get_digits_as_number(&digits, start);
            if is_prime(number) {
                found_primes.push(number);
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            for j in 0..found_primes.len() {
                found_circular_primes.insert(found_primes[j]);
            }
        }
    }

    println!("Number of circular primes: {}", found_circular_primes.len())
}
