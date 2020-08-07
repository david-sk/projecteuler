#
# Largest palindrome product, v3
# https://projecteuler.net/problem=4
#
# A palindromic number reads the same both ways. The largest palindrome made from
# the product of two 2-digit numbers is 9009 = 91 × 99.
# Find the largest palindrome made from the product of two 3-digit numbers.
#


def is_palindromic(n):
    n_as_string = str(n)
    i = 0
    j = len(n_as_string) - 1
    while i <= j and n_as_string[i] == n_as_string[j]:
        i += 1
        j -= 1
    return i > j


def find_largest_palindrome_product(num_digits=3):
    MAX_MULTIPLIER = 10 ** num_digits - 1
    MIN_MULTIPLIER = 10 ** (num_digits - 1)

    a = b = MAX_MULTIPLIER
    while not is_palindromic(a * b):
        if b > a:
            b -= 1
        else:
            if b == a:
                b = MAX_MULTIPLIER
            a -= 1
            if a < MIN_MULTIPLIER:
                return -1  # no existing palindrome

    palindrome = a * b

    if b > a:
        a_limit = b
        a = MAX_MULTIPLIER
        while not is_palindromic(a * b):
            if a * b > palindrome:
                b -= 1
            elif a < a_limit:
                return palindrome
            else:
                b = a
                a -= 1

    if is_palindromic(a * b):
        palindrome = a * b

    return palindrome


def run():
    print('Largest palindrome:', find_largest_palindrome_product(num_digits=3))
