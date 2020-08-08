#
# Combinatoric selections, v1
# https://projecteuler.net/problem=53
#

from math import factorial


def run():
    combinatoric_selections_sum = 0
    for n in range(1, 101):
        for r in range(1, n + 1):
            if factorial(n) // (factorial(r) * factorial(n - r)) > 1000000:
                combinatoric_selections_sum += 1
    print('Sum:', combinatoric_selections_sum)
