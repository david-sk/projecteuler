#
# Multiples of 3 and 5, v1
# https://projecteuler.net/problem=1
#
# If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
# The sum of these multiples is 23.
# Find the sum of all the multiples of 3 or 5 below 1000.
#


def run():
    multiples_sum = sum(i for i in range(1, 1001) if i % 3 == 0 or i % 5 == 0)
    print('Multiples of 3 or 5 sum:', multiples_sum)
