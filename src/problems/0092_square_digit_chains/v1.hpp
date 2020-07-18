//
// Square digit chains, v1
// https://projecteuler.net/problem=92
//
// A number chain is created by continuously adding the square of the digits in a number to form a
// new number until it has been seen before.
// For example,
// 44 → 32 → 13 → 10 → 1 → 1
// 85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
// Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop.
// What is most amazing is that EVERY starting number will eventually arrive at 1 or 89.
// How many starting numbers below ten million will arrive at 89?
//

#include <cmath>
#include <iostream>

namespace Problem_92_V1 {

void run() {
    unsigned int limit = 10000000;

    unsigned int num_89_numbers = 0;

    for (unsigned int i = 1; i < limit; i++) {
        unsigned int n = i;
        while (n != 1 && n != 89) {
            unsigned int digits_square_sum = 0;
            while (n > 0) {
                digits_square_sum += pow(n % 10, 2);
                n /= 10;
            }
            n = digits_square_sum;
        }
        if (n == 89) {
            num_89_numbers++;
        }
    }

    std::cout << "Number of 89-numbers: " << num_89_numbers << std::endl;
}

}  // namespace Problem_92_V1
