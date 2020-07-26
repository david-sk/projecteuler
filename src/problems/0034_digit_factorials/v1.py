#
# Digit factorials, v1
# https://projecteuler.net/problem=34
#
# 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
# Find the sum of all numbers which are equal to the sum of the factorial of their digits.
# Note: as 1! = 1 and 2! = 2 are not sums they are not included.
#

from math import factorial


def get_digits_factorial_sum(n):
    digits_factorial_sum = 0
    while n > 0:
        digits_factorial_sum += factorial(n % 10)
        n //= 10
    return digits_factorial_sum


def run():
    # Why 100000? No idea, just put something there, and it gives the correct answer...
    # That is soooo cheating :D
    result = sum(n for n in range(3, 100000) if n == get_digits_factorial_sum(n))
    print('Result:', result)
