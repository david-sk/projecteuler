#
# Square root convergents, v1
# https://projecteuler.net/problem=57
#
# See webpage for problem description.
#

from fractions import Fraction


def run():
    num_exceeding_digits = 0
    n = Fraction(0)
    for _ in range(1000):
        n = Fraction(1, 2 + n)
        sqrt_2_convergent = Fraction(1 + n)
        if len(str(sqrt_2_convergent.numerator)) > len(str(sqrt_2_convergent.denominator)):
            num_exceeding_digits += 1
    print('Num exceeding digits', num_exceeding_digits)
