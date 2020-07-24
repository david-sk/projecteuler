//
// Bouncy numbers, v1
// https://projecteuler.net/problem=112
//
// Working from left-to-right if no digit is exceeded by the digit to its left it is called an
// increasing number; for example, 134468.
// Similarly if no digit is exceeded by the digit to its right it is called a decreasing number;
// for example, 66420.
// We shall call a positive integer that is neither increasing nor decreasing a "bouncy" number;
// for example, 155349.
// Clearly there cannot be any bouncy numbers below one-hundred, but just over half of the numbers
// below one-thousand (525) are bouncy. In fact, the least number for which the proportion of
// bouncy numbers first reaches 50% is 538.
// Surprisingly, bouncy numbers become more and more common and by the time we reach 21780 the
// proportion of bouncy numbers is equal to 90%.
// Find the least number for which the proportion of bouncy numbers is exactly 99%.
//

#include <iostream>

namespace Problem_112_V1 {

bool is_bouncy(unsigned long n) {
    int direction = -1, next_direction;  // -1: unset, 1: lower than, 2: greater than
    int prev_digit, digit = n % 10;
    n /= 10;
    do {
        prev_digit = digit;
        digit = n % 10;
        n /= 10;
        if (prev_digit != digit) {
            next_direction = prev_digit < digit ? 1 : 2;
            if (direction != -1 && direction != next_direction) {
                return true;
            }
            direction = next_direction;
        }
    } while (n > 0);
    return false;
}

void run() {
    unsigned long n = 0, num_bouncy = 0;
    const unsigned int wanted_percentage = 99;
    unsigned int percentage = 0;

    while (percentage < wanted_percentage) {
        n++;
        if (is_bouncy(n)) {
            num_bouncy++;
        }
        percentage = num_bouncy * 100 / n;
    }

    std::cout << "Least number: " << n << std::endl;
}

}  // namespace Problem_112_V1
