//
// Integer right triangles, v1
// https://projecteuler.net/problem=39
//
// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are
// exactly three solutions for p = 120.
//     {20,48,52}, {24,45,51}, {30,40,50}
// For which value of p ≤ 1000, is the number of solutions maximised?
//

use std::collections::HashMap;

pub fn run() {
    let limit = 1000;
    let mut solutions: HashMap<u64, u64> = HashMap::new();

    for i in 1..=limit {
        let i_square = i * i;
        for j in i..=limit {
            let end = limit - i - j;
            if end < 1 || end > limit {
                continue;
            }
            let ij_square = i_square + j * j;
            for k in 1..=end {
                if ij_square == k * k {
                    let key = i + j + k;
                    if solutions.contains_key(&key) {
                        solutions.insert(key, solutions[&key] + 1);
                    } else {
                        solutions.insert(key, 1);
                    }
                }
            }
        }
    }

    let (mut max_key, mut max_value) = (0, 0);
    for (key, value) in solutions {
        if value > max_value {
            max_key = key;
            max_value = value;
        }
    }

    println!("Result: {}", max_key);
}
