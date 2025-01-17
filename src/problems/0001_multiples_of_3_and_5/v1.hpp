//
// Multiples of 3 and 5, v1
// https://projecteuler.net/problem=1
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.
//

#include <iostream>

namespace Problem_1_V1 {

void run() {
    unsigned int sum = 0;
    for (unsigned int i = 1; i < 1000; i++) {
        if (i % 3 == 0 || i % 5 == 0) {
            sum += i;
        }
    }
    std::cout << "Multiples of 3 or 5 sum: " << sum << std::endl;
}

}  // namespace Problem_1_V1
