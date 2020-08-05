//
// Odd elimination, v1
// https://projecteuler.net/problem=539
//
// Start from an ordered list of all integers from 1 to n. Going from left to right, remove the
// first number and every other number afterward until the end of the list. Repeat the procedure
// from right to left, removing the right most number and every other number from the numbers left.
// Continue removing every other numbers, alternating left to right and right to left, until a
// single number remains.
// Starting with n = 9, we have:
//     1 2 3 4 5 6 7 8 9
//     2 4 6 8
//     2 6
//     6
// Let P(n) be the last number left starting with a list of length n.
// Let S(n) = sum(P(k) for k in 1..=n).
// You are given P(1) = 1, P(9) = 6, P(1000) = 510, S(1000) = 268271.
// Find S(10^18) mod 987654321.
//

pub fn run() {
    // that works but would of course takes far too long for 10^18, this is more for debug purposes
    let mut sum = 1;
    for n in (2..=1000).step_by(2) {
        let mut a = vec![];
        for i in 1..=n {
            a.push(i);
        }
        while a.len() != 1 {
            let length = a.len();
            for i in (1..length).step_by(2) {
                a.insert(length, a[i]);
            }
            a.drain(..length);
        }
        sum += a[0] * if n != 1000 { 2 } else { 1 };
        for i in 1..=n {
            print!("{} ", i);
        }
        println!(" -->  {}\n", a[0]);
    }
    println!("Sum: {}", sum);
}
