#
# Power digit sum, v1
# https://projecteuler.net/problem=16
#
# 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
# What is the sum of the digits of the number 2^1000?
#


def run():
    digits_sum = sum(int(c) for c in str(2 ** 1000))
    print('Sum of digits:', digits_sum)
