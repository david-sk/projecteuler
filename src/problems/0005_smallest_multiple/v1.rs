//
// Smallest multiple, v1
// https://projecteuler.net/problem=5
//
// 2520 is the smallest number that can be divided by each of the numbers
// from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
//

pub fn run() {
    let n = 20;
    let mut multiple = 0;
    loop {
        multiple += n;
        let mut ok = true;
        for i in 2..n {
            if multiple % i != 0 {
                ok = false;
                break;
            }
        }
        if ok {
            break;
        }
    }
    println!("Smallest multiple: {}", multiple);
}
