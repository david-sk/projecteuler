//
// Lychrel numbers, v1
// https://projecteuler.net/problem=55
//
// If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
// Not all numbers produce palindromes so quickly. For example,
//     349 + 943 = 1292,
//     1292 + 2921 = 4213
//     4213 + 3124 = 7337
// That is, 349 took three iterations to arrive at a palindrome.
// Although no one has proved it yet, it is thought that some numbers, like 196, never produce
// a palindrome. A number that never forms a palindrome through the reverse and add process is
// called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose
// of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition
// you are given that for every number below ten-thousand, it will either (i) become a palindrome
// in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has
// managed so far to map it to a palindrome. In fact, 10677 is the first number to be shown to
// require over fifty iterations before producing a palindrome: 4668731596684224866951378664
// (53 iterations, 28-digits).
// Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first
// example is 4994.
// How many Lychrel numbers are there below ten-thousand?
//

// NOTE: this returns the reverse value of `n`, and also whether `n` is palindromic
fn get_reverse_n(mut n: i128) -> (i128, bool) {
    let mut reverse_n = 0;
    let mut digits = vec![];
    while n > 0 {
        let n_mod_10 = n % 10;
        digits.push(n_mod_10);
        reverse_n = reverse_n * 10 + n_mod_10;
        n /= 10;
    }
    let num_digits = digits.len();
    for i in 0..num_digits {
        if digits[i] != digits[num_digits - i - 1] {
            return (reverse_n, false);
        }
    }
    return (reverse_n, true);
}

fn is_lychrel(mut n: i128) -> bool {
    for i in 0..50 {
        let (reverse_n, is_palindromic) = get_reverse_n(n);
        if i > 0 && is_palindromic {
            return false;
        }
        n += reverse_n;
    }
    // NOTE: `reverse_n` value is not needed here, so we could split the `get_reverse_n`
    // function to only return whether it's palindromic to slightly increase efficiency
    let (_, is_palindromic) = get_reverse_n(n);
    return !is_palindromic;
}

pub fn run() {
    let limit = 10000;
    let mut num_lychrel_numbers = 0;
    for i in 0..limit {
        if is_lychrel(i) {
            num_lychrel_numbers += 1;
        }
    }
    println!("Num Lychrel numbers: {}", num_lychrel_numbers);
}
