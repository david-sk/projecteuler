#
# Large non-Mersenne prime, v1
# https://projecteuler.net/problem=97
#
# The first known prime found to exceed one million digits was discovered in 1999, and is a
# Mersenne prime of the form 2^6972593−1; it contains exactly 2,098,960 digits. Subsequently
# other Mersenne primes, of the form 2^p−1, have been found which contain more digits.
# However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207
# digits: 28433×2^7830457+1.
# Find the last ten digits of this prime number.
#


def run():
    # That's totally cheating with Python! :)
    price_number_last_ten_digits = str(28433 * 2 ** 7830457 + 1)[-10:]
    print(price_number_last_ten_digits)
