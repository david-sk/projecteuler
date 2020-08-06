//
// Pandigital prime, v1
// https://projecteuler.net/problem=41
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
// exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
// What is the largest n-digit pandigital prime that exists?
//

mod prime;

use prime::is_prime;

// Note that this takes the next _lower_ permutation of digits
fn next_permutation(array: &mut [u64], length: usize) -> bool {
    let mut i: usize = length - 1;
    while i > 0 && array[i] >= array[i - 1] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j: usize = i - 1;
    let mut k: usize = length - 1;
    while array[k] > array[j] {
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

fn get_digits_as_number(digits: &mut [u64], length: usize) -> u64 {
    let mut number = 0;
    let mut ten_powers = 1;
    for i in 0..length {
        number += digits[length - i - 1] * ten_powers;
        ten_powers *= 10;
    }
    return number;
}

fn is_digits_number_prime(digits: &mut [u64], length: usize) -> bool {
    if length > 1 && (digits[length - 1] % 2 == 0 || digits[length - 1] == 5) {
        return false;
    }
    return is_prime(get_digits_as_number(digits, length));
}

pub fn run() {
    const TOTAL_LENGTH: usize = 9;
    let mut digits: [u64; TOTAL_LENGTH] = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut length = TOTAL_LENGTH;

    while !is_digits_number_prime(&mut digits, length) {
        if !next_permutation(&mut digits, length) && length > 1 {
            length -= 1;
            for i in 0..length {
                digits[i] = (length - i) as u64;
            }
        }
    }

    let largest_pandigital_prime = get_digits_as_number(&mut digits, length);
    println!("Largest pandigital prime: {}", largest_pandigital_prime);
}
