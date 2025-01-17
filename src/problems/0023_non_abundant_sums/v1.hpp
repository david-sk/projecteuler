//
// Non-abundant sums, v1
// https://projecteuler.net/problem=23
//
// A perfect number is a number for which the sum of its proper divisors is exactly equal to the
// number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
// which means that 28 is a perfect number.
// A number n is called deficient if the sum of its proper divisors is less than n and it is called
// abundant if this sum exceeds n.
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
// written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
// all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
// upper limit cannot be reduced any further by analysis even though it is known that the greatest
// number that cannot be expressed as the sum of two abundant numbers is less than this limit.
// Find the sum of all the positive integers which cannot be written as the sum of two abundant
// numbers.
//

#include <cmath>
#include <iostream>
#include <vector>

namespace Problem_23_V1 {

unsigned int getProperDivisorsSum(unsigned int n) {
    unsigned int sum = 1;
    for (unsigned int i = 2; i <= sqrt(n); i++) {
        if (n % i == 0) {
            sum += i + (i * i != n ? n / i : 0);
        }
    }
    return sum;
}

void run() {
    unsigned int limit = 28123;

    std::vector<unsigned int> abundantNumbers;
    for (unsigned int i = 1; i < limit; i++) {
        if (getProperDivisorsSum(i) > i) {
            abundantNumbers.push_back(i);
        }
    }
    unsigned int abundantNumbersSize = abundantNumbers.size();

    unsigned int nonAbundantSum = 0;
    for (unsigned int i = 1; i < limit; i++) {
        bool isSumOfTwoAbundantNumbers = false;
        for (unsigned int j = 0; j < abundantNumbersSize; j++) {
            if (abundantNumbers[j] >= i) {
                break;
            }
            for (unsigned int k = j; k < abundantNumbersSize; k++) {
                if (abundantNumbers[j] + abundantNumbers[k] == i) {
                    isSumOfTwoAbundantNumbers = true;
                    j = abundantNumbersSize;  // breaks outer loop
                    break;
                }
                if (abundantNumbers[j] + abundantNumbers[k] > i) {
                    break;
                }
            }
        }
        if (!isSumOfTwoAbundantNumbers) {
            nonAbundantSum += i;
        }
    }

    std::cout << "Non abundant sum: " << nonAbundantSum << std::endl;
}

}  // namespace Problem_23_V1
