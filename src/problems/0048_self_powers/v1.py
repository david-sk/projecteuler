#
# Self powers, v1
# https://projecteuler.net/problem=48
#
# The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
# Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
#


def run():
    # That's totally cheating with Python! :)
    self_powers_sum_last_10_digits = str(sum(i ** i for i in range(1, 1001)))[-10:]
    print(self_powers_sum_last_10_digits)