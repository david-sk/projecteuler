//
// How many reversible numbers are there below one-billion?, v1
// https://projecteuler.net/problem=145
//
// Some positive integers n have the property that the sum [ n + reverse(n) ] consists entirely
// of odd (decimal) digits. For instance, 36 + 63 = 99 and 409 + 904 = 1313. We will call such
// numbers reversible; so 36, 63, 409, and 904 are reversible. Leading zeroes are not allowed in
// either n or reverse(n).
// There are 120 reversible numbers below one-thousand.
// How many reversible numbers are there below one-billion (10^9)?
//

#include <iostream>

namespace Problem_145_V1 {

unsigned long get_reverse_n(unsigned long n) {
    unsigned long reverse_n = 0;
    while (n > 0) {
        reverse_n = reverse_n * 10 + n % 10;
        n /= 10;
    }
    return reverse_n;
}

bool is_reversible(unsigned long n) {
    unsigned long result = n + get_reverse_n(n);
    while (result > 0) {
        if ((result % 10) % 2 == 0) {
            return false;
        }
        result /= 10;
    }
    return true;
}

void run() {
    const unsigned long limit = 1000000000;
    unsigned long num_reversible = 0;

    for (unsigned long i = 1; i < limit; i++) {
        if (i % 10 != 0 && is_reversible(i)) {
            num_reversible++;
        }
    }

    std::cout << "Num reversible: " << num_reversible << std::endl;
}

}  // namespace Problem_145_V1
