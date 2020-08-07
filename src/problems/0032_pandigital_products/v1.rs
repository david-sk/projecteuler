//
// Pandigital products, v1
// https://projecteuler.net/problem=32
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
// exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
// multiplier, and product is 1 through 9 pandigital.
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as
// a 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only include it once
// in your sum.
//

use std::collections::HashSet;

fn next_permutation(array: &mut [u64], length: usize) -> bool {
    let mut i: usize = length - 1;
    while i > 0 && array[i] <= array[i - 1] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j: usize = i - 1;
    let mut k: usize = length - 1;
    while array[k] < array[j] {
        k -= 1;
    }
    let tmp = array[j];
    array[j] = array[k];
    array[k] = tmp;

    j = length - 1;
    k = i;
    while k < j {
        let tmp = array[j];
        array[j] = array[k];
        array[k] = tmp;
        k += 1;
        j -= 1;
    }

    return true;
}

fn get_digits_as_number(digits: &mut [u64], start: usize, length: usize) -> u64 {
    let mut number = 0;
    let mut ten_powers = 1;
    for n in 0..length {
        number += digits[start + n] * ten_powers;
        ten_powers *= 10;
    }
    return number;
}

pub fn run() {
    const LENGTH: usize = 9;
    let mut digits: [u64; LENGTH] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut found_numbers: HashSet<u64> = HashSet::new();
    let mut sum = 0;

    loop {
        for i in 1..=(LENGTH - 2) {
            let a = get_digits_as_number(&mut digits, 0, i);
            for j in i..=(LENGTH - i - 1) {
                let c = get_digits_as_number(&mut digits, i + j, LENGTH - i - j);
                if found_numbers.contains(&c) {
                    continue;
                }
                let b = get_digits_as_number(&mut digits, i, j);
                if a * b == c {
                    sum += c;
                    found_numbers.insert(c);
                }
            }
        }
        if !next_permutation(&mut digits, LENGTH) {
            break;
        }
    }

    println!("Sum: {}", sum);
}
