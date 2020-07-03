//
// 10-substrings, v1
// https://projecteuler.net/problem=529
//
// A 10-substring of a number is a substring of its digits that sum to 10.
// For example, the 10-substrings of the number 3523014 are:
//     [352]3014
//     3[523]014
//     3[5230]14
//     35[23014]
// A number is called 10-substring-friendly if every one of its digits belongs to a 10-substring.
// For example, 3523014 is 10-substring-friendly, but 28546 is not.
// Let T(n) be the number of 10-substring-friendly numbers from 1 to 10^n (inclusive).
// For example T(2) = 9 and T(5) = 3492.
// Find T(10^18) mod 1 000 000 007.
//

// TODO: trying stuff here, need to find a proper solution

fn checks(n: i64) -> bool {
    // let value_of_sum: i64 = 10; // FIXME: make use of it???
    let n_string = n.to_string();

    let mut start = 1;
    let mut sum = n_string[0..1].parse::<i64>().unwrap();
    for i in 1..=n_string.len() {
        println!("{}", n_string[0..1].parse::<i64>().unwrap());
        sum += n_string[i - 1..i].parse::<i64>().unwrap();
        println!(">, {}", sum);
        while sum > 10 {
            sum -= n_string[start - 1..start].parse::<i64>().unwrap();

            start += 1;
        }
        if sum == 10 {
            println!("!, {}", sum);
            return true;
        }
    }

    return false;
}

pub fn run() {
    println!("{}", checks(28546));
}
