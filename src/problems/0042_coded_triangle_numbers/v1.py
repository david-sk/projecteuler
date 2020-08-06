#
# Coded triangle numbers, v1
# https://projecteuler.net/problem=42
#
# The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten
# triangle numbers are:
#     1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
# By converting each letter in a word to a number corresponding to its alphabetical position and
# adding these values we form a word value.
# For example, the word value for SKY is 19 + 11 + 25 = 55 = t10.
# If the word value is a triangle number then we shall call the word a triangle word.
# Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
# two-thousand common English words, how many are triangle words?
#
# NOTE: words.txt file is put in this problem's folder.
#

from os.path import dirname, join


def get_words():
    with open(join(dirname(__file__), 'words.txt')) as f:
        return f.read().replace('"', '').split(',')


def run():
    coded_words = [sum(ord(c) - 64 for c in word) for word in get_words()]
    max_coded_words_value = max(coded_words)

    n = 1
    triangles = set()
    next_triangle = 1  # obvious that the first triangle number is 1
    while next_triangle <= max_coded_words_value:
        triangles.add(next_triangle)
        n += 1
        next_triangle = n * (n + 1) // 2

    num_triangle_words = sum(coded_word in triangles for coded_word in coded_words)

    print('Number of triangle words:', num_triangle_words)
