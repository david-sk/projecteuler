//
// Lattice paths, v1
// https://projecteuler.net/problem=15
//
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
// there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?
//

fn lattice_paths(end: &[u64; 2], cursor: [u64; 2], counter: u64) -> u64 {
    if (cursor[0], cursor[1]) != (end[0], end[1]) {
        if cursor[0] == end[0] {
            return lattice_paths(end, [cursor[0], cursor[1] + 1], counter);
        } else if cursor[1] == end[1] {
            return lattice_paths(end, [cursor[0] + 1, cursor[1]], counter);
        }
        return lattice_paths(end, [cursor[0], cursor[1] + 1], counter)
            + lattice_paths(end, [cursor[0] + 1, cursor[1]], counter);
    }
    return counter + 1;
}

pub fn run() {
    let end: [u64; 2] = [20, 20];
    let final_counter = lattice_paths(&end, [0, 0], 0);
    println!("Result: {}", final_counter);
}
