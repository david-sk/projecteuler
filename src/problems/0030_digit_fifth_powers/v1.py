#
# Digit fifth powers, v1
# https://projecteuler.net/problem=30
#
# Surprisingly there are only three numbers that can be written as the sum of fourth
# powers of their digits:
#     1634 = 1^4 + 6^4 + 3^4 + 4^4
#     8208 = 8^4 + 2^4 + 0^4 + 8^4
#     9474 = 9^4 + 4^4 + 7^4 + 4^4
# As 1 = 1^4 is not a sum it is not included.
# The sum of these numbers is 1634 + 8208 + 9474 = 19316.
# Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
#


def get_digits_power_sum(n):
    digits_power_sum = 0
    while n > 0:
        digits_power_sum += (n % 10) ** 5
        n //= 10
    return digits_power_sum


def run():
    # Why 1000000? No idea, just put something there, and it gives the correct answer...
    # That is soooo cheating :D
    result = sum(n for n in range(3, 1000000) if n == get_digits_power_sum(n))
    print('Result:', result)
