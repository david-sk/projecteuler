#
# Powerful digit counts, v1
# https://projecteuler.net/problem=63
#
# The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number,
# 134217728=8^9, is a ninth power.
# How many n-digit positive integers exist which are also an nth power?
#


def run():
    print('Answer', sum(len(str(a ** b)) == b for b in range(1, 23) for a in range(1, 11)))
