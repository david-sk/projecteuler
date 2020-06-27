//
// Special Pythagorean triplet, v1
// https://projecteuler.net/problem=9
//
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//     a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
//

pub fn run() {
    let n = 1000;
    for a in 0..=n {
        for b in (a + 1)..=(n - a) {
            for c in (b + 1)..=(n - b) {
                if a + b + c == n && a * a + b * b == c * c {
                    println!("Pythagorean triplet product:");
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    return;
                } else if a + b + c > n {
                    break;
                }
            }
        }
    }
    println!("No Pythagorean triplet product found");
}
