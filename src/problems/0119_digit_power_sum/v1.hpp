//
// Digit power sum, v1
// https://projecteuler.net/problem=119
//
// The number 512 is interesting because it is equal to the sum of its digits raised to some power:
// 5 + 1 + 2 = 8, and 8^3 = 512. Another example of a number with this property is 614656 = 28^4.
// We shall define a_n to be the nth term of this sequence and insist that a number must contain at
// least two digits to have a sum.
// You are given that a_2 = 512 and a_10 = 614656.
// Find a_30.
//

#include <cmath>
#include <iostream>
#include <set>

namespace Problem_119_V1 {

const unsigned int power_sums_table_limit = 100;  // 100 is more than enough for this problem!
unsigned long long power_sums_table[power_sums_table_limit][power_sums_table_limit];
std::set<unsigned long long> power_sums_set;

void setup_power_sums_values() {
    for (unsigned int i = 2; i < power_sums_table_limit; i++) {
        for (unsigned int exponent = 1; exponent < power_sums_table_limit; exponent++) {
            power_sums_table[i][exponent] = pow(i, exponent);
            power_sums_set.insert(power_sums_table[i][exponent]);
        }
    }
}

unsigned int get_digits_sum(unsigned long long n) {
    unsigned int digits_sum = 0;
    while (n > 0) {
        digits_sum += n % 10;
        n /= 10;
    }
    return digits_sum;
}

bool is_digit_power_sum(unsigned long long n) {
    unsigned int digits_sum = get_digits_sum(n);
    if (digits_sum == 1) {
        return false;
    }
    if (digits_sum < power_sums_table_limit) {
        for (unsigned int j = 0; j < power_sums_table_limit; j++) {
            if (power_sums_table[digits_sum][j] >= n) {
                return power_sums_table[digits_sum][j] == n;
            }
        }
    }
    std::cout << "Power sums table limit reached, should not happen!" << std::endl;
    exit(1);
}

void run() {
    setup_power_sums_values();

    const unsigned int limit = 30;
    unsigned int num_found_digit_power_sum = 0;
    unsigned long long n = 9;  // initialize at 9 to start looking for n >= 10 as next value

    std::set<unsigned long long>::iterator iter;

    while (num_found_digit_power_sum < limit) {
        unsigned long long next_n = 0;
        for (iter = power_sums_set.begin(); iter != power_sums_set.end(); iter++) {
            if (*iter > n && (next_n == 0 || next_n > *iter)) {
                next_n = *iter;
            }
        }
        if (next_n > 0) {
            n = next_n;
        } else {
            std::cout << "Could not find next value for `n`, should not happen!" << std::endl;
            exit(1);
        }
        if (is_digit_power_sum(n)) {
            num_found_digit_power_sum++;
        }
    }

    std::cout << "Digit power sum result: " << n << std::endl;
}

}  // namespace Problem_119_V1
