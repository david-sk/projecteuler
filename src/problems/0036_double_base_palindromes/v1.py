#
# Double-base palindromes, v1
# https://projecteuler.net/problem=36
#
# The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
# Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
# (Please note that the palindromic number, in either base, may not include leading zeros.)
#


def is_palindrome(n, base=10):
    if n < base:
        return n >= 0
    digits = []
    divided_n = n
    while divided_n > 0:
        digits.append(divided_n % base)
        divided_n //= base
    num_digits = len(digits)
    return all(digits[i] == digits[num_digits - i - 1] for i in range(num_digits))


def run():
    limit = 1000000
    palindromic_base_10_and_2_sum = sum(
        i for i in range(1, limit) if is_palindrome(i) and is_palindrome(i, base=2)
    )
    print('Palindromic base 10 and 2 sum:', palindromic_base_10_and_2_sum)
