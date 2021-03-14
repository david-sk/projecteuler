#
# Distinct primes factors, v1
# ttps://projecteuler.net/problem=47
#
# The first two consecutive numbers to have two distinct prime factors are:
# 14 = 2 × 7
# 15 = 3 × 5
# The first three consecutive numbers to have three distinct prime factors are:
# 644 = 2^2 × 7 × 23
# 645 = 3 × 5 × 43
# 646 = 2 × 17 × 19.
# Find the first four consecutive integers to have four distinct prime factors each.
# What is the first of these numbers?
#

from math import sqrt


def is_prime(n):
    if n < 3 or n % 2 == 0:
        return n == 2
    for i in range(3, int(sqrt(n)) + 1, 2):
        if n % i == 0:
            return False
    return True


def find_num_prime_factors(n, primes):
    half_n = n // 2
    num_prime_factors = 0
    for prime in primes:
        if n % prime == 0:
            num_prime_factors += 1
        elif prime > half_n:
            break
    return num_prime_factors


def extend_primes(primes, chunk_amount):
    n = primes[-1] + 2
    i = chunk_amount
    while i > 0:
        if is_prime(n):
            primes.append(n)
            i -= 1
        n += 2


def run():
    NUM_CONSECUTIVE = 4
    PRIME_CHUNK = 1000

    primes = [2, 3]
    extend_primes(primes, PRIME_CHUNK)

    n = 1
    while True:
        if all(
            find_num_prime_factors(i, primes) == NUM_CONSECUTIVE
            for i in range(n, n + NUM_CONSECUTIVE)
        ):
            print('Answer', n)
            break
        n += 1
        if primes[-1] < n:
            extend_primes(primes, PRIME_CHUNK)
