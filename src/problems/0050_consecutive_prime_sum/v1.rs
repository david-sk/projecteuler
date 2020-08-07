//
// Consecutive prime sum, v1
// https://projecteuler.net/problem=50
//
// The prime 41, can be written as the sum of six consecutive primes:
//     41 = 2 + 3 + 5 + 7 + 11 + 13
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21
// terms, and is equal to 953.
// Which prime, below one-million, can be written as the sum of the most consecutive primes?
//

mod prime;

use prime::is_prime;

// While this gives the answer under a minute (well, more or less depending on the computer),
// this brute force solution is pretty terrible. Okay for a v1, but a v2 would be welcome!
pub fn run() {
    let limit = 1000000;

    let mut primes = vec![2];
    for i in (3..limit).step_by(2) {
        if is_prime(i) {
            primes.push(i);
        }
    }

    let primes_length = primes.len();
    let mut consecutive_length = primes_length - 1;
    while consecutive_length > 1 {
        for i in 0..=(primes_length - consecutive_length) {
            let mut sum = 0;
            for j in i..(consecutive_length + i) {
                sum += primes[j];
                if sum >= limit {
                    sum = 0;
                    break;
                }
            }
            if is_prime(sum) {
                println!("Result: {}", sum);
                return;
            }
        }
        consecutive_length -= 1;
    }

    println!("Nothing found :(")
}
