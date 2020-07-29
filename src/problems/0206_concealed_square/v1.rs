//
// Concealed Square, v1
// https://projecteuler.net/problem=206
//
// Find the unique positive integer whose square has the form 1_2_3_4_5_6_7_8_9_0,
// where each “_” is a single digit.
//

fn has_correct_digits(mut n: u64) -> bool {
    let mut i = 0;
    let mut next_correct_digit = 9;
    while n > 0 {
        if i % 2 == 1 {
            if n % 10 != next_correct_digit {
                return false;
            }
            next_correct_digit -= 1;
        }
        n /= 10;
        i += 1;
    }
    return true;
}

pub fn run() {
    let from = (1020304050607080900 as u64 as f64).sqrt() as u64;
    let to = (1929394959697989990 as u64 as f64).sqrt() as u64 + 1;
    for i in from..=to {
        if i % 10 == 0 && has_correct_digits(i * i / 10) {
            println!("Result: {}", i);
            return;
        }
    }
    println!("Result not found :(");
}
