#
# Cubic permutations, v1
# https://projecteuler.net/problem=62
#
# The cube, 41063625 (345^3), can be permuted to produce two other cubes: 56623104 (384^3)
# and 66430125 (405^3). In fact, 41063625 is the smallest cube which has exactly three
# permutations of its digits which are also cube.
# Find the smallest cube for which exactly five permutations of its digits are cube.
#

from collections import defaultdict


def get_sorted_digits(n):
    digits = []
    while n > 0:
        value = n % 10
        n //= 10
        for i in range(len(digits)):
            if value < digits[i]:
                digits.insert(i, value)
                break
        else:
            digits.append(value)
    return digits


def run():
    digits_map = defaultdict(list)
    key = ''

    n = 1
    while True:
        key = ''.join(str(i) for i in get_sorted_digits(n ** 3))
        digits_map[key].append(n)
        if len(digits_map[key]) == 5:
            break
        n += 1

    print('Smallest cube:', digits_map[key][0] ** 3)
