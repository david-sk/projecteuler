//
// Multiples of 3 and 5, v1
// https://projecteuler.net/problem=1
//
// Here is a dummy version (we start at n = 0 instead of 1, which is useless)
// for the example of having multiple problem versions.
//

pub mod solver {
    pub fn run() {
        let mut sum = 0;
        for n in 0..1000 {
            if n % 3 == 0 || n % 5 == 0 {
                sum += n
            }
        }
        println!("Sum: {}", sum);
    }
}
