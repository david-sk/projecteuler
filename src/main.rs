//
// Entry point.
//
// Have only one uncommented path that will be used
// for running one of the Project Euler problem.
//
// NOTE:
// This assumes all problems have a solver module with a run function.
//

#[path = 
    // "problems/1_multiples_of_3_and_5/v1.rs"
    // "problems/1_multiples_of_3_and_5/v2.rs"
    "problems/2_even_fibonacci_numbers/v1.rs"
] mod problem;

fn main() {
    problem::solver::run();
}
