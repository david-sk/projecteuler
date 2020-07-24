#include <time.h>

#include <iostream>

#include "problems/0001_multiples_of_3_and_5/v1.hpp"
#include "problems/0002_even_fibonacci_numbers/v1.hpp"
#include "problems/0023_non_abundant_sums/v1.hpp"
#include "problems/0092_square_digit_chains/v1.hpp"
#include "problems/0112_bouncy_numbers/v1.hpp"

void displayUsage() {
    std::cout << "\nUsage: ./cpp_projecteuler [problem number] [filename (without extension)]"
              << std::endl;
    std::cout << "For example: `./cpp_projecteuler 1 v1`\n"
              << std::endl;
    std::cout << "(This usage assumes it is all compiled to a `cpp_projecteuler` executable)\n"
              << std::endl;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        displayUsage();
        return -1;
    }

    std::string problemNumber = std::string(argv[1]);
    std::string problemVersion = std::string(argv[2]);

    std::cout << "--------------------------------------------------------------\n"
              << std::endl;

    clock_t now = clock();

    if (problemNumber == "1" && problemVersion == "v1") {
        Problem_1_V1::run();
    } else if (problemNumber == "2" && problemVersion == "v1") {
        Problem_2_V1::run();
    } else if (problemNumber == "23" && problemVersion == "v1") {
        Problem_23_V1::run();
    } else if (problemNumber == "92" && problemVersion == "v1") {
        Problem_92_V1::run();
    } else if (problemNumber == "112" && problemVersion == "v1") {
        Problem_112_V1::run();
    } else {
        displayUsage();
    }

    double elapsed = (double)(clock() - now) / CLOCKS_PER_SEC;

    printf("\n~ Duration: %.9f seconds ~\n", elapsed);

    std::cout << "\n--------------------------------------------------------------" << std::endl;

    return 0;
}
