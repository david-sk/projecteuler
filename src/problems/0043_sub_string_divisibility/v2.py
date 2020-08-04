#
# Sub-string divisibility, v2
# https://projecteuler.net/problem=43
#
# The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits
# 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
# Let d_1 be the 1st digit, d_2 be the 2nd digit, and so on. In this way, we note the following:
#     d_2 d_3 d_4 = 406 is divisible by 2
#     d_3 d_4 d_5 = 063 is divisible by 3
#     d_4 d_5 d_6 = 635 is divisible by 5
#     d_5 d_6 d_7 = 357 is divisible by 7
#     d_6 d_7 d_8 = 572 is divisible by 11
#     d_7 d_8 d_9 = 728 is divisible by 13
#     d_8 d_9 d_10 = 289 is divisible by 17
# Find the sum of all 0 to 9 pandigital numbers with this property.
#


def next_permutation(array, length):
    i = length - 1
    while i > 0 and array[i] <= array[i - 1]:
        i -= 1
    if i == 0:
        return False

    j = i - 1
    k = length - 1
    while array[k] < array[j]:
        k -= 1
    array[j], array[k] = array[k], array[j]

    j = length - 1
    k = i
    while k < j:
        array[j], array[k] = array[k], array[j]
        k += 1
        j -= 1

    return True


def run():
    pandigital_numbers_sum = 0

    digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    divisors = (2, 3, 5, 7, 11, 13, 17)
    length = len(digits)

    # not checking the first permutation as we know 123 is not divisible by 2 (conveninent :-P)
    while next_permutation(digits, length):
        if (
            digits[3] % 2 == 0
            and digits[5] in (0, 5)
            and digits[-1] % 2 == 1
            and all(
                (digits[i] * 100 + digits[i + 1] * 10 + digits[i + 2]) % n == 0
                for i, n in enumerate(divisors, 1)
            )
        ):
            pandigital_numbers_sum += sum(
                digit * (10 ** i) for i, digit in enumerate(reversed(digits))
            )

    print('Pandigital numbers sum:', pandigital_numbers_sum)
