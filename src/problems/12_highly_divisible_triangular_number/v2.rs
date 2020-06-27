//
// Highly divisible triangular number, v2
// https://projecteuler.net/problem=12
//
// The sequence of triangle numbers is generated by adding the natural numbers.
// So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28.
// The first ten terms would be:
//     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
// Let us list the factors of the first seven triangle numbers:
//      1: 1
//      3: 1,3
//      6: 1,2,3,6
//     10: 1,2,5,10
//     15: 1,3,5,15
//     21: 1,3,7,21
//     28: 1,2,4,7,14,28
// We can see that 28 is the first triangle number to have over five divisors.
// What is the value of the first triangle number to have over five hundred divisors?
//

fn get_num_divisors(n: i64) -> i64 {
    let mut num_divisors = 0;
    let sqrt = (n as f64).sqrt() as i64;
    for i in 1..=sqrt {
        if n % i == 0 {
            num_divisors += if i == sqrt { 1 } else { 2 };
        }
    }
    return num_divisors;
}

pub fn run() {
    let n = 500;

    let mut triangle_number = 0;

    let mut i = 0;
    loop {
        i += 1;
        triangle_number += i;
        if get_num_divisors(triangle_number) > n {
            break;
        }
    }

    println!("Triangle number: {}", triangle_number);
}
