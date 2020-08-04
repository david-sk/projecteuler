//
// Sub-string divisibility, v3
// https://projecteuler.net/problem=43
//
// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits
// 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
// Let d_1 be the 1st digit, d_2 be the 2nd digit, and so on. In this way, we note the following:
//     d_2 d_3 d_4 = 406 is divisible by 2
//     d_3 d_4 d_5 = 063 is divisible by 3
//     d_4 d_5 d_6 = 635 is divisible by 5
//     d_5 d_6 d_7 = 357 is divisible by 7
//     d_6 d_7 d_8 = 572 is divisible by 11
//     d_7 d_8 d_9 = 728 is divisible by 13
//     d_8 d_9 d_10 = 289 is divisible by 17
// Find the sum of all 0 to 9 pandigital numbers with this property.
//

fn next_permutation(array: &mut [u64], length: usize) -> bool {
    let mut i: usize = length - 1;
    while i > 0 && array[i] <= array[i - 1] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }

    let mut j: usize = i - 1;
    let mut k: usize = length - 1;
    while array[k] < array[j] {
        k -= 1;
    }
    let tmp = array[j];
    array[j] = array[k];
    array[k] = tmp;

    j = length - 1;
    k = i;
    while k < j {
        let tmp = array[j];
        array[j] = array[k];
        array[k] = tmp;
        k += 1;
        j -= 1;
    }

    return true;
}

pub fn run() {
    let mut pandigital_numbers_sum = 0;

    const LENGTH: usize = 10;
    let mut digits: [u64; LENGTH] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    const NUM_DIVISORS: usize = 7;
    let divisors: [u64; NUM_DIVISORS] = [2, 3, 5, 7, 11, 13, 17];

    // not checking the first permutation as we know 123 is not divisible by 2 (conveninent :-P)
    while next_permutation(&mut digits, LENGTH) {
        if digits[LENGTH - 1] % 2 != 1 {
            continue;
        }
        let mut ok = true;
        let mut j: usize = 1;
        for i in 0..NUM_DIVISORS {
            if (digits[j] * 100 + digits[j + 1] * 10 + digits[j + 2]) % divisors[i] != 0 {
                ok = false;
                break;
            }
            j += 1;
        }
        if ok {
            let mut ten_powers = 1;
            for i in 0..LENGTH {
                pandigital_numbers_sum += digits[LENGTH - i - 1] * ten_powers;
                ten_powers *= 10;
            }
        }
    }

    println!("Pandigital numbers sum: {}", pandigital_numbers_sum);
}
