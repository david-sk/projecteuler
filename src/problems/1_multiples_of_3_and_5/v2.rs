//
// Multiples of 3 and 5, v2
// https://projecteuler.net/problem=1
//

pub mod solver {
    pub fn run() {
        let mut sum = 0;
        for n in 1..1000 {
            if n % 3 == 0 || n % 5 == 0 {
                sum += n
            }
        }
        println!("Sum: {}", sum);
    }
}
