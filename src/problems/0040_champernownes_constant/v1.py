#
# Champernowne's constant, v1
# https://projecteuler.net/problem=40
#
# An irrational decimal fraction is created by concatenating the positive integers:
#     0.12345678910[1]112131415161718192021...
# It can be seen that the 12th digit of the fractional part is 1.
# If d_n represents the nth digit of the fractional part,
# find the value of the following expression.
# d_1 × d_10 × d_100 × d_1000 × d_10000 × d_100000 × d_1000000
#


def run():
    LIMIT = 1000000

    number = 0
    result = 1
    digits = ''
    digits_length = 0
    wanted_digit = 1

    while wanted_digit <= LIMIT:
        number += 1
        number_as_str = str(number)
        digits += number_as_str
        digits_length += len(number_as_str)
        if wanted_digit <= digits_length:
            result *= int(digits[wanted_digit - 1])
            wanted_digit *= 10

    print('result:', result)
