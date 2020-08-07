//
// Digit cancelling fractions, v1
// https://projecteuler.net/problem=33
//
// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to
// simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by
// cancelling the 9s.
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
// There are exactly four non-trivial examples of this type of fraction, less than one in value,
// and containing two digits in the numerator and denominator.
// If the product of these four fractions is given in its lowest common terms, find the value of
// the denominator.
//

fn gcd(a: u64, b: u64) -> u64 {
    return if b == 0 { a } else { gcd(b, a % b) };
}

pub fn run() {
    let (mut numerator_product, mut denominator_product) = (1, 1);
    for i in 1..=9 {
        for j in 1..=9 {
            for i2 in 1..=9 {
                for j2 in 1..=9 {
                    let (a, b, c, d) = (i as f64, j as f64, i2 as f64, j2 as f64);
                    let value: f64 = (a * 10.0 + b) / (c * 10.0 + d);
                    if value >= 1.0 {
                        continue;
                    } else if (a == d && value == b / c) || (b == d && value == a / c) {
                        numerator_product *= if a == d { b } else { a } as u64;
                        denominator_product *= c as u64;
                    } else if (a == c && value == b / d) || (b == c && value == a / d) {
                        numerator_product *= if a == d { b } else { a } as u64;
                        denominator_product *= d as u64;
                    }
                }
            }
        }
    }
    let result = denominator_product / gcd(numerator_product, denominator_product);
    println!("Result: {}", result);
}
