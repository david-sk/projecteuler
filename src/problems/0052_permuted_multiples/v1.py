#
# Permuted multiples, v1
# https://projecteuler.net/problem=52
#
# It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits,
# but in a different order.
# Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
#

from collections import Counter


def get_digits_counter(n):
    digits = []
    while n > 0:
        digits.append(n % 10)
        n //= 10
    return Counter(digits)


def run():
    n = 1

    while not (
        get_digits_counter(2 * n)
        == get_digits_counter(3 * n)
        == get_digits_counter(4 * n)
        == get_digits_counter(5 * n)
        == get_digits_counter(6 * n)
    ):
        n += 1

    print('Result:', n)
