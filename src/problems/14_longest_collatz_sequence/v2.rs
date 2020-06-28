//
// Longest Collatz sequence, v2
// https://projecteuler.net/problem=14
//
// The following iterative sequence is defined for the set of positive integers:
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting
// numbers finish at 1.
// Which starting number, under one million, produces the longest chain?
// NOTE: Once the chain starts the terms are allowed to go above one million.
//

pub fn run() {
    const STARTING_NUMBER_LIMIT: usize = 1000000;

    let mut longest_chain = 0;
    let mut starting_number_for_longest_chain = 0;

    // array that increases memory usage but makes algorithm run much faster than v1
    let mut chain_sizes: [u32; STARTING_NUMBER_LIMIT] = [0; STARTING_NUMBER_LIMIT];

    for starting_number in 1..STARTING_NUMBER_LIMIT {
        let mut n: i128 = starting_number as i128;
        let mut counter: u32 = 1;
        while n != 1 {
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            if n < STARTING_NUMBER_LIMIT as i128 && chain_sizes[(n - 1) as usize] > 0 {
                counter += chain_sizes[(n - 1) as usize];
                break;
            }
            counter += 1;
        }

        chain_sizes[starting_number - 1] = counter;

        if counter > longest_chain {
            longest_chain = counter;
            starting_number_for_longest_chain = starting_number;
        }
    }

    println!(
        "Starting number for longest chain: {}",
        starting_number_for_longest_chain
    );
}
