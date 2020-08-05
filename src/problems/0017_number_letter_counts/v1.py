#
# Number letter counts, v1
# https://projecteuler.net/problem=17
#
# If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
# 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
# If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
# letters would be used?
# NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
# letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
# numbers is in compliance with British usage.
#

#
# NOTE: This Python solution is pretty ugly, but this problem is pretty boring so ¯\_(ツ)_/¯
#


def run():
    numbers = {
        1: 'one',
        2: 'two',
        3: 'three',
        4: 'four',
        5: 'five',
        6: 'six',
        7: 'seven',
        8: 'eight',
        9: 'nine',
        10: 'ten',
        11: 'eleven',
        12: 'twelve',
        13: 'thirteen',
        14: 'fourteen',
        15: 'fifteen',
        16: 'sixteen',
        17: 'seventeen',
        18: 'eighteen',
        19: 'nineteen',
        20: 'twenty',
        30: 'thirty',
        40: 'forty',
        50: 'fifty',
        60: 'sixty',
        70: 'seventy',
        80: 'eighty',
        90: 'ninety',
    }
    HUNDRED = 'hundred'
    THOUSAND = 'thousand'
    AND = 'and'

    s = 0
    for i in range(1, 1001):
        if i < 20:
            s += len(numbers[i])
        elif i < 100:
            s += len(numbers[i - i % 10]) + (len(numbers[i % 10]) if i % 10 != 0 else 0)
        elif i < 1000:
            s += len(numbers[i // 100]) + len(HUNDRED)
            if i % 100 == 0:
                continue
            s += len(AND)
            if i % 100 < 20:
                s += len(numbers[i % 100]) if i % 100 != 0 else 0
            else:
                s += len(numbers[i // 10 % 10 * 10]) + (len(numbers[i % 10]) if i % 10 != 0 else 0)
        else:
            s += len(numbers[1]) + len(THOUSAND)

    print('Letters sum:', s)
