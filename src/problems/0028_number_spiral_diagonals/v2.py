#
# Number spiral diagonals, v2
# https://projecteuler.net/problem=28
#
# Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is
# formed as follows:
#     21 22 23 24 25
#     20  7  8  9 10
#     19  6  1  2 11
#     18  5  4  3 12
#     17 16 15 14 13
# It can be verified that the sum of the numbers on the diagonals is 101.
# What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
#


def run():
    SIZE = 1001
    diagonals_sum = 0
    direction = 1  # 1 to 4 (inclusive)
    increment = 1
    number = 0

    while increment < SIZE or direction == 1:
        number += increment
        diagonals_sum += number if direction == 1 else number + 1
        direction = direction % 4 + 1
        increment += direction % 2

    print('Diagonals sum:', diagonals_sum)
