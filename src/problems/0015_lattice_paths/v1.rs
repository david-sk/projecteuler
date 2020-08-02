//
// Lattice paths, v1
// https://projecteuler.net/problem=15
//
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
// there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?
//

fn lattice_paths((x, y): (u64, u64), (max_x, max_y): (u64, u64)) -> u64 {
    if (x, y) == (max_x, max_y) {
        return 1; // found path!
    }
    return if x < max_x { lattice_paths((x + 1, y), (max_x, max_y)) } else { 0 } // going deeper!
        + if y < max_y { lattice_paths((x, y + 1), (max_x, max_y)) } else { 0 };
}

pub fn run() {
    println!("Result: {}", lattice_paths((0, 0), (20, 20)));
}
