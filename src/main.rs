//
// Entry point. Usage is explained in `display_usage` function.
//

// 0001 to 0009 problems
#[path = "problems/0001_multiples_of_3_and_5/v1.rs"]
mod problem_1_v1;
#[path = "problems/0001_multiples_of_3_and_5/v2.rs"]
mod problem_1_v2;
#[path = "problems/0002_even_fibonacci_numbers/v1.rs"]
mod problem_2_v1;
#[path = "problems/0003_largest_prime_factor/v1.rs"]
mod problem_3_v1;
#[path = "problems/0004_largest_palindrome_product/v1.rs"]
mod problem_4_v1;
#[path = "problems/0004_largest_palindrome_product/v2.rs"]
mod problem_4_v2;
#[path = "problems/0005_smallest_multiple/v1.rs"]
mod problem_5_v1;
#[path = "problems/0006_sum_square_difference/v1.rs"]
mod problem_6_v1;
#[path = "problems/0007_10001st_prime/v1.rs"]
mod problem_7_v1;
#[path = "problems/0008_largest_product_in_a_series/v1.rs"]
mod problem_8_v1;
#[path = "problems/0009_special_pythagorean_triplet/v1.rs"]
mod problem_9_v1;

// 00** problems
#[path = "problems/0010_summation_of_primes/v1.rs"]
mod problem_10_v1;
#[path = "problems/0011_largest_product_in_a_grid/v1.rs"]
mod problem_11_v1;
#[path = "problems/0012_highly_divisible_triangular_number/v1.rs"]
mod problem_12_v1;
#[path = "problems/0012_highly_divisible_triangular_number/v2.rs"]
mod problem_12_v2;
#[path = "problems/0013_large_sum/v1.rs"]
mod problem_13_v1;
#[path = "problems/0014_longest_collatz_sequence/v1.rs"]
mod problem_14_v1;
#[path = "problems/0014_longest_collatz_sequence/v2.rs"]
mod problem_14_v2;
#[path = "problems/0015_lattice_paths/v1.rs"]
mod problem_15_v1;
#[path = "problems/0018_maximum_path_sum_I/v1.rs"]
mod problem_18_v1;
#[path = "problems/0031_coins_sum/v1.rs"]
mod problem_31_v1;
#[path = "problems/0032_pandigital_products/v1.rs"]
mod problem_32_v1;
#[path = "problems/0033_digit_cancelling_fractions/v1.rs"]
mod problem_33_v1;
#[path = "problems/0039_integer_right_triangles/v1.rs"]
mod problem_39_v1;
#[path = "problems/0041_pandigital_prime/v1.rs"]
mod problem_41_v1;
#[path = "problems/0043_sub_string_divisibility/v3.rs"]
mod problem_43_v3;
#[path = "problems/0049_prime_permutations/v1.rs"]
mod problem_49_v1;
#[path = "problems/0055_lychrel_numbers/v1.rs"]
mod problem_55_v1;
#[path = "problems/0059_xor_decryption/v1.rs"]
mod problem_59_v1;

// 0*** problems
#[path = "problems/0179_consecutive_positive_divisors/v1.rs"]
mod problem_179_v1;
#[path = "problems/0206_concealed_square/v1.rs"]
mod problem_206_v1;
#[path = "problems/0346_strong_repunits/v1.rs"]
mod problem_346_v1;
#[path = "problems/0357_prime_generating_integers/v1.rs"]
mod problem_357_v1;
#[path = "problems/0529_10_substrings/v1.rs"]
mod problem_529_v1;
#[path = "problems/0539_odd_elimination/v1.rs"]
mod problem_539_v1;

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
        ("15", "v1") => problem_15_v1::run(),
        ("18", "v1") => problem_18_v1::run(),
        ("31", "v1") => problem_31_v1::run(),
        ("32", "v1") => problem_32_v1::run(),
        ("33", "v1") => problem_33_v1::run(),
        ("39", "v1") => problem_39_v1::run(),
        ("41", "v1") => problem_41_v1::run(),
        ("43", "v3") => problem_43_v3::run(),
        ("49", "v1") => problem_49_v1::run(),
        ("55", "v1") => problem_55_v1::run(),
        ("59", "v1") => problem_59_v1::run(),
        ("179", "v1") => problem_179_v1::run(),
        ("206", "v1") => problem_206_v1::run(),
        ("346", "v1") => problem_346_v1::run(),
        ("357", "v1") => problem_357_v1::run(),
        ("529", "v1") => problem_529_v1::run(),
        ("539", "v1") => problem_539_v1::run(),
        _ => display_usage(),
    }
    let elapsed = now.elapsed();
    println!("\n~ Duration: {} seconds ~", elapsed.as_secs_f64());
    println!("\n--------------------------------------------------------------");
}
