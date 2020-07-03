#
# Number spiral diagonals, v1
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

    pos_x, pos_y = SIZE / 2, SIZE / 2

    previous_counter = 1
    counter = 1
    is_x_turn = True
    increment = 1

    for i in range(1, SIZE * SIZE + 1):
        if pos_x == pos_y or pos_x + pos_y == SIZE:
            diagonals_sum += i

        if is_x_turn:
            pos_x += increment
        else:
            pos_y += increment

        counter -= 1
        if counter == 0:
            if not is_x_turn:
                is_x_turn = True
                increment = -increment
                counter = previous_counter + 1
                previous_counter = counter
            else:
                is_x_turn = False
                counter = previous_counter

    print('Diagonals sum:', diagonals_sum)
