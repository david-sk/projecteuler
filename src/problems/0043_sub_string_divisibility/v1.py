#
# Sub-string divisibility, v1
# https://projecteuler.net/problem=43
#
# The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits
# 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
# Let d_1 be the 1st digit, d_2 be the 2nd digit, and so on. In this way, we note the following:
#     d_2 d_3 d_4 = 406 is divisible by 2
#     d_3 d_4 d_5 = 063 is divisible by 3
#     d_4 d_5 d_6 = 635 is divisible by 5
#     d_5 d_6 d_7 = 357 is divisible by 7
#     d_6 d_7 d_8 = 572 is divisible by 11
#     d_7 d_8 d_9 = 728 is divisible by 13
#     d_8 d_9 d_10 = 289 is divisible by 17
# Find the sum of all 0 to 9 pandigital numbers with this property.
#

from itertools import permutations


def run():
    pandigital_numbers_sum = sum(
        sum(digit * (10 ** i) for i, digit in enumerate(reversed(digits)))
        for digits in permutations(range(10))
        if (
            digits[-1] % 2 == 1
            and all(
                (digits[i] * 100 + digits[i + 1] * 10 + digits[i + 2]) % n == 0
                for i, n in enumerate((2, 3, 5, 7, 11, 13, 17), 1)
            )
        )
    )
    print('Pandigital numbers sum:', pandigital_numbers_sum)
