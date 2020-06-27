//
// Largest palindrome product, v2
// https://projecteuler.net/problem=4
//
// A palindromic number reads the same both ways. The largest palindrome made from
// the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
//

pub fn run() {
    // The idea is to looks at all possibilities of a * b, ordered by the largest product first
    // and try to find an algorithms that starts with the largest product then finds the second
    // largest product, then the third largest, and so on.
    // In a complexity that is of course lower than the first version (v1.rs).
    //
    // Below are a/b values differences when going down the "good" vector.
    //
    // =========
    // a       b
    // =========
    // -1      0
    // 0      -1 -----
    // -1      1
    // 0      -1 -----
    // -1      1
    // 1      -2 -----
    // -1      1
    // -1      1
    // 1      -2 -----
    // -1      1
    // -1      1
    // 2      -3 -----
    // -1      1
    // -1      1
    // -1      1
    // 2      -3 -----
    // -1      1
    // -1      1
    // -1      1
    // 3      -4 -----
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // 3      -4 -----
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // 4      -5 -----
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // 4      -5 -----
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // 5      -6 -----
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // -1      1
    // 5      -6 -----
    // -1      1

    let mut good = vec![];
    for i in 100..1000 {
        for j in i..1000 {
            good.push((i, j, i * j));
        }
    }
    good.sort_by(|(_, _, product_a), (_, _, product_b)| product_b.partial_cmp(product_a).unwrap());

    let (mut a, mut b) = (999, 999);

    let mut increase_amount = 0;
    let mut increase_counter = 0;
    let mut decrease_counter = 1;

    let mut compare = vec![(a, b, a * b)];
    while a >= 100 && b >= 100 {
        if decrease_counter > 0 {
            a -= 1;
            b += if b < 999 { 1 } else { 0 };
            decrease_counter -= 1;
        } else {
            a += increase_amount;
            b -= increase_amount + 1;
            increase_counter += 1;
            decrease_counter = increase_amount + 1;
            if increase_counter == 2 {
                increase_counter = 0;
                increase_amount += 1;
            }
        }
        compare.push((a, b, a * b))
    }

    // check if the algorithm is good (TODO: if it is, use it to find the largest palindrome!)
    let mut index = 0;
    while index < good.len() && index < compare.len() {
        if good[index] != compare[index] {
            for i in (index - 10)..(index + 50) {
                let (compare_a, compare_b, _) = compare[i];
                let (good_a, good_b, _) = good[i];
                if i == index {
                    println!("");
                }
                println!("{}, {}   |   {}, {}", compare_a, compare_b, good_a, good_b);
                if i == index {
                    println!("");
                }
            }
            println!("\nNOT GOOD AT INDEX: {}", index);
            return;
        }
        index += 1;
    }
    if good.len() != compare.len() {
        println!("ALL EQUAL BUT DIFFERENT LENGTHS!");
    } else {
        println!("ALL GOOD");
    }
}
