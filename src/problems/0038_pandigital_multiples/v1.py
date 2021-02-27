#
# Pandigital multiples, v1
# https://projecteuler.net/problem=38
#
# Take the number 192 and multiply it by each of 1, 2, and 3:
# 192 × 1 = 192
# 192 × 2 = 384
# 192 × 3 = 576
# By concatenating each product we get the 1 to 9 pandigital, 192384576.
# We will call 192384576 the concatenated product of 192 and (1, 2, 3)
# The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
# giving the pandigital, 918273645, which is the concatenated product of 9 and (1, 2, 3, 4, 5).
# What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated
# product of an integer with (1, 2, ..., n) where n > 1?
#


def is_pandigital(n):
    digits = []
    while n > 0:
        digit = n % 10
        if digit == 0 or digit in digits:
            return False
        digits.append(digit)
        n //= 10
    return True


def run():
    # The max number to multiply cannot be above 10000 (5 digits) otherwise even with the smallest
    # possibility `number concatenated to (number * 2)` we would get over 9 digits.
    # Also the number to multiply is of course a pandigital itself.
    pandigitals = [i for i in range(1, 10000) if is_pandigital(i)]

    # Then we take only the 9 digits pandigital canditates (so between 100000000 and 1000000000),
    # add them to a list `valid_candidates`, and the answer is the max value in that list.
    valid_candidates = []
    for pandigital in pandigitals:
        candidate = pandigital
        i = 2
        while is_pandigital(candidate) and candidate < 1000000000:
            if candidate > 100000000:
                valid_candidates.append(candidate)
            candidate = int(str(candidate) + str(pandigital * i))
            i += 1

    print('Largest pandigital:', max(valid_candidates))
