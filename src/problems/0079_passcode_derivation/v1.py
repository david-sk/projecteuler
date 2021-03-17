#
# Passcode derivation, v1
# https://projecteuler.net/problem=79
#
# A common security method used for online banking is to ask the user for three random characters
# from a passcode. For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th
# characters; the expected reply would be: 317.
# The text file, keylog.txt, contains fifty successful login attempts.
# Given that the three characters are always asked for in order, analyse the file so as to
# determine the shortest possible secret passcode of unknown length.
#
# The keylog.txt content is directly put into `ATTEMPTS`
#

from itertools import permutations

ATTEMPTS = [
    '319',
    '680',
    '180',
    '690',
    '129',
    '620',
    '762',
    '689',
    '762',
    '318',
    '368',
    '710',
    '720',
    '710',
    '629',
    '168',
    '160',
    '689',
    '716',
    '731',
    '736',
    '729',
    '316',
    '729',
    '729',
    '710',
    '769',
    '290',
    '719',
    '680',
    '318',
    '389',
    '162',
    '289',
    '162',
    '718',
    '729',
    '319',
    '790',
    '680',
    '890',
    '362',
    '319',
    '760',
    '316',
    '729',
    '380',
    '319',
    '728',
    '716',
]


def is_possible(passcode):
    for a, b, c in ATTEMPTS:
        if a not in passcode or b not in passcode or c not in passcode:
            return False
        a_index, c_index = passcode.index(a), passcode.rindex(c)
        if b not in passcode[a_index + 1 : c_index]:
            return False
    return True


def find_shortest_passcode(digits):
    queue = []
    queue.append(digits)

    discovered = set()
    discovered.add(digits)

    while len(queue) > 0:
        digits = queue.pop()

        for possibility in permutations(digits):
            possibility_as_str = ''.join(possibility)
            if is_possible(possibility_as_str):
                return possibility_as_str

        for digit in digits:
            new_digits = digits + digit
            if new_digits not in discovered:
                discovered.add(new_digits)
                queue.append(new_digits)

    return None


def run():
    digits = ''
    for attempt in ATTEMPTS:
        digits += ''.join(c for c in attempt if c not in digits)

    print('Answer', find_shortest_passcode(digits))
