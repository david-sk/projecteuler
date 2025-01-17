#
# Spiral Primes, v1
# https://projecteuler.net/problem=58
#
# Starting with 1 and spiralling anticlockwise in the following way,
# a square spiral with side length 7 is formed.
# 37 36 35 34 33 32 31
# 38 17 16 15 14 13 30
# 39 18  5  4  3 12 29
# 40 19  6  1  2 11 28
# 41 20  7  8  9 10 27
# 42 21 22 23 24 25 26
# 43 44 45 46 47 48 49
# It is interesting to note that the odd squares lie along the bottom right diagonal, but what is
# more interesting is that 8 out of the 13 numbers lying along both diagonals are prime; that is,
# a ratio of 8/13 ≈ 62%.
# If one complete new layer is wrapped around the spiral above, a square spiral with side length 9
# will be formed. If this process is continued, what is the side length of the square spiral for
# which the ratio of primes along both diagonals first falls below 10%?
#

from math import sqrt


def is_prime(n):
    if n < 3 or n % 2 == 0:
        return n == 2
    for i in range(3, int(sqrt(n)) + 1, 2):
        if n % i == 0:
            return False
    return True


def run():
    RATIO_THRESHOLD = 0.1
    num_primes_found = 0
    num_diagonal_numbers_visited = 0
    direction = 1  # 1 to 4 (inclusive)
    increment = 1
    number = 0

    while True:
        number += increment
        if is_prime(number if direction == 1 else number + 1):
            num_primes_found += 1
        direction = direction % 4 + 1
        increment += direction % 2
        num_diagonal_numbers_visited += 1
        if (
            num_diagonal_numbers_visited > 1
            and num_diagonal_numbers_visited % 4 == 1
            and num_primes_found / num_diagonal_numbers_visited < RATIO_THRESHOLD
        ):
            break

    print('Spiral side length:', num_diagonal_numbers_visited // 2 + 1)
