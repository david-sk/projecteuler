//
// Largest palindrome product, v1
// https://projecteuler.net/problem=4
//
// A palindromic number reads the same both ways. The largest palindrome made from
// the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
//

mod palindrome;

use palindrome::is_palindrome;

pub fn run() {
    let mut largest_palindrome = -1;

    for i in 100..1000 {
        for j in i..1000 {
            let product = i * j;
            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    if largest_palindrome == -1 {
        println!("No largest palindrome found");
    } else {
        println!("Largest palindrome: {}", largest_palindrome);
    }
}
