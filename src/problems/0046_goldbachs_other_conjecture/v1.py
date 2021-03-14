#
# Goldbach's other conjecture, v1
# https://projecteuler.net/problem=46
#
# It was proposed by Christian Goldbach that every odd composite number can be
# written as the sum of a prime and twice a square.
# 9 = 7 + 2×1^2
# 15 = 7 + 2×2^2
# 21 = 3 + 2×3^2
# 25 = 7 + 2×3^2
# 27 = 19 + 2×2^2
# 33 = 31 + 2×1^2
# It turns out that the conjecture was false.
# What is the smallest odd composite that cannot be written as
# the sum of a prime and twice a square?
#

from math import sqrt


def is_prime(n):
    if n < 3 or n % 2 == 0:
        return n == 2
    for i in range(3, int(sqrt(n)) + 1, 2):
        if n % i == 0:
            return False
    return True


def generate_primes(a, b):
    return [i for i in range(a, b + 1, 2) if is_prime(i)]


def find_odd_composite_without_sum():
    PRIME_CHUNK = 1000
    primes = [2] + generate_primes(3, 3 + PRIME_CHUNK)

    odd_composite = 9

    while True:
        if primes[-1] < odd_composite:
            from_odd_number = primes[-1] + 2
            primes.extend(generate_primes(from_odd_number, from_odd_number + PRIME_CHUNK))
        for prime in primes:
            # prime + 2 * n^2 == odd_composite
            n = 1
            while prime + 2 * n ** 2 < odd_composite:
                n += 1
            if prime + 2 * n ** 2 == odd_composite:
                break
        else:  # nobreak
            return odd_composite

        odd_composite += 2
        while odd_composite in primes:
            odd_composite += 2


def run():
    print('Answer', find_odd_composite_without_sum())
