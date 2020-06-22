//
// Even Fibonacci numbers, v1
// https://projecteuler.net/problem=2
//

pub mod solver {
    pub fn run() {
        let mut sum = 0;
        let (mut a, mut b) = (1, 2);
        while b < 4000000 {
            if b % 2 == 0 {
                sum += b
            }
            b = a + b;
            a = b - a;
        }
        println!("Sum: {}", sum);
    }
}
