//
// Coins sum, v1
// https://projecteuler.net/problem=31
//
// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins
// in general circulation:
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
// It is possible to make £2 in the following way:
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// How many different ways can £2 be made using any number of coins?
//

fn is_total_reached(coins: &mut [u64], total: u64) -> bool {
    return coins[1]
        + 2 * coins[2]
        + 5 * coins[5]
        + 10 * coins[10]
        + 20 * coins[20]
        + 50 * coins[50]
        + 100 * coins[100]
        == total;
}

pub fn run() {
    let total = 200;
    let mut coins: [u64; 101] = [0; 101];
    let mut num_ways = 1; // it's obvious there is only one way with 200

    // GIVES THE ANSWER "FAST" (FOR BRUTE FORCE..) BUT THIS IS VERY UGLY!!!
    for i1 in 0..=(total / 1) {
        coins[1] = i1;
        for i2 in 0..=(total / 2) {
            coins[2] = i2;
            for i5 in 0..=(total / 5) {
                coins[5] = i5;
                for i10 in 0..=(total / 10) {
                    coins[10] = i10;
                    for i20 in 0..=(total / 20) {
                        coins[20] = i20;
                        for i50 in 0..=(total / 50) {
                            coins[50] = i50;
                            for i100 in 0..=(total / 100) {
                                coins[100] = i100;
                                if is_total_reached(&mut coins, total) {
                                    num_ways += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Number of ways: {}", num_ways);
}
