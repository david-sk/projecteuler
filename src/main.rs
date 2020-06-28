//
// Entry point. Usage is explained in `display_usage` function.
//

#[path = "problems/1_multiples_of_3_and_5/v1.rs"]
mod problem_1_v1;
#[path = "problems/1_multiples_of_3_and_5/v2.rs"]
mod problem_1_v2;
#[path = "problems/2_even_fibonacci_numbers/v1.rs"]
mod problem_2_v1;
#[path = "problems/3_largest_prime_factor/v1.rs"]
mod problem_3_v1;
#[path = "problems/4_largest_palindrome_product/v1.rs"]
mod problem_4_v1;
#[path = "problems/4_largest_palindrome_product/v2.rs"]
mod problem_4_v2;
#[path = "problems/5_smallest_multiple/v1.rs"]
mod problem_5_v1;
#[path = "problems/6_sum_square_difference/v1.rs"]
mod problem_6_v1;
#[path = "problems/7_10001st_prime/v1.rs"]
mod problem_7_v1;
#[path = "problems/8_largest_product_in_a_series/v1.rs"]
mod problem_8_v1;
#[path = "problems/9_special_pythagorean_triplet/v1.rs"]
mod problem_9_v1;

#[path = "problems/10_summation_of_primes/v1.rs"]
mod problem_10_v1;
#[path = "problems/11_largest_product_in_a_grid/v1.rs"]
mod problem_11_v1;
#[path = "problems/12_highly_divisible_triangular_number/v1.rs"]
mod problem_12_v1;
#[path = "problems/12_highly_divisible_triangular_number/v2.rs"]
mod problem_12_v2;
#[path = "problems/13_large_sum/v1.rs"]
mod problem_13_v1;
#[path = "problems/14_longest_collatz_sequence/v1.rs"]
mod problem_14_v1;
#[path = "problems/14_longest_collatz_sequence/v2.rs"]
mod problem_14_v2;

use std::env;
use std::time::Instant;

fn display_usage() {
    println!("Usage: cargo run [problem number] [filename (without extension)]");
    println!("For example: `cargo run 1 v1`");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("\n/!\\ Not enough arguments /!\\\n");
        display_usage();
        return;
    }
    println!("--------------------------------------------------------------\n");
    let now = Instant::now();
    match (&args[1][..], &args[2][..]) {
        ("1", "v1") => problem_1_v1::run(),
        ("1", "v2") => problem_1_v2::run(),
        ("2", "v1") => problem_2_v1::run(),
        ("3", "v1") => problem_3_v1::run(),
        ("4", "v1") => problem_4_v1::run(),
        ("4", "v2") => problem_4_v2::run(),
        ("5", "v1") => problem_5_v1::run(),
        ("6", "v1") => problem_6_v1::run(),
        ("7", "v1") => problem_7_v1::run(),
        ("8", "v1") => problem_8_v1::run(),
        ("9", "v1") => problem_9_v1::run(),
        ("10", "v1") => problem_10_v1::run(),
        ("11", "v1") => problem_11_v1::run(),
        ("12", "v1") => problem_12_v1::run(),
        ("12", "v2") => problem_12_v2::run(),
        ("13", "v1") => problem_13_v1::run(),
        ("14", "v1") => problem_14_v1::run(),
        ("14", "v2") => problem_14_v2::run(),
        _ => display_usage(),
    }
    let elapsed = now.elapsed();
    println!("\n~ Duration: {} seconds ~", elapsed.as_secs_f64());
    println!("\n--------------------------------------------------------------");
}
