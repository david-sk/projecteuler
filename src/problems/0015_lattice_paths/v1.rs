//
// Lattice paths, v1
// https://projecteuler.net/problem=15
//
// Starting in the top left corner of a 2×2 grid, and only being able to move to the right and
// down, there are exactly 6 routes to the bottom right corner.
// How many such routes are there through a 20×20 grid?
//

// TODO: trying stuff here, need to find a proper solution

pub fn run() {
    const SIZE: usize = 3 * 2;
    let n = SIZE / 2;
    let mut num_routes = 1;

    let mut array = [false; SIZE];
    for i in 0..n {
        array[i] = true;
    }

    // 2 -> 2    f(n - 4)
    // 3 -> 3    f(n - 3)
    // 4 -> 6    f(n - 2)
    // 5 -> 10   f(n - 1)
    // 6 -> 24   f(n)

    // -
    // 2 -> 2
    // T,F
    // F,T

    // - -
    // 3 -> 3
    // T,T,F
    // T,F,T
    // F,T,T

    // - -
    // - -
    // 4 -> 6
    // T,T,F,F
    // T,F,T,F
    // T,F,F,T
    // F,T,T,F
    // F,T,F,T
    // F,F,T,T

    // - - -
    // - - -
    // 5 -> 10
    // T,T,T,F,F
    // T,T,F,T,F
    // T,T,F,F,T
    // T,F,T,T,F
    // T,F,T,F,T
    // T,F,F,T,T
    // F,T,T,T,F
    // F,T,T,F,T
    // F,T,F,T,T
    // F,F,T,T,T

    // 6 -> 24
    // T,F,T,T,F,F
    // T,F,T,F,T,F
    // T,F,T,F,F,T
    // T,F,F,T,T,F
    // T,F,F,T,F,T
    // T,F,F,F,T,T
    // F,T,T,T,F,F
    // F,T,T,F,T,F
    // F,T,T,F,F,T
    // F,T,F,T,T,F
    // F,T,F,T,F,T
    // F,T,F,F,T,T
    // F,F,T,T,T,F
    // F,F,T,T,F,T
    // F,F,T,F,T,T
    // F,F,F,T,T,T
    // T,T,F,F,F,T
    // T,T,F,F,T,F
    // T,T,F,T,F,F
    // T,T,T,F,F,F
    // T,F,T,T,T,F
    // T,T,F,T,F,T
    // F,T,T,F,T,T
    // ?

    // def f(n):
    //     num_routes = 0
    //     block = [[False for _ in range(n + 1)] for _ in range(n + 1)]

    //     while True:
    //         i = 0
    //         j = 0
    //         has_been_blocked = False
    //         while not (i == n and j == n):
    //             if i + 1 <= n and not block[i + 1][j]:
    //                 i += 1
    //             else:
    //                 if not has_been_blocked:
    //                     block[i][j] = True
    //                     has_been_blocked = True
    //                 j += 1
    //         print(block)
    //         num_routes += 1

    // f(2)

    //   012
    // 0 FFT
    // 1 FFF
    // 2 FFF

    //   012
    // 0 FFT
    // 1 FFT
    // 2 FFF

    //   012
    // 0 FTT
    // 1 FFT
    // 2 FFF

    //   012
    // 0 FTT
    // 1 FTT
    // 2 FFF

    //   012
    // 0 FTT
    // 1 FTT
    // 2 FFT

    let mut start = 0;
    loop {
        let mut i = start;
        while array[i] {
            i += 1;
            if i == SIZE {
                println!("COINCOIN {}", num_routes);
                return;
            }
        }
        if i == start {
            start += 1;
            continue;
        }
        array[i] = !array[i];
        array[i - 1] = !array[i - 1];
        num_routes += 1;
    }

    // println!("Number of routes: {}", num_routes);
}
