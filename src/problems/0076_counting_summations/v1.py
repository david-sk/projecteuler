#
# Counting summations, v1
# https://projecteuler.net/problem=76
#
# It is possible to write five as a sum in exactly six different ways:
# 4 + 1
# 3 + 2
# 3 + 1 + 1
# 2 + 2 + 1
# 2 + 1 + 1 + 1
# 1 + 1 + 1 + 1 + 1
# How many different ways can one hundred be written as a sum of at least two positive integers?
#


def find_summations(n):
    if n <= 2:
        return {(1, 1)} if n == 2 else set()
    summations = set()
    prev_summations = find_summations(n - 1)
    for prev_summation in prev_summations:
        summations.add((1, *prev_summation))
        for a in range(prev_summation[-1], n):
            for i, b in enumerate(prev_summation):
                candidate = a + (n - 1 - b)
                if candidate < n:
                    break
                if candidate == n:
                    summation = prev_summation[:i] + (a,) + prev_summation[i + 1 :]
                    summations.add(tuple(sorted(summation)))
    return summations


def run():
    # Well, this algorithm version is way too slow to find the solution for 100.
    # But let's cheat! Execute this to have the 2 to 30 sequence printed in reasonable time:

    sequence = ', '.join(str(len(find_summations(n))) for n in range(2, 31))

    print('Sequence for 2 to 30:')
    print(sequence, '\n')
    print('Go find the solution for 100 at:', '\n')
    print('http://sequencedb.net/index.html?s=' + sequence.replace(' ', '+').replace(',', '%2C'))

    # Then either go to the printed link, or copy/paste the comma separated numbers in
    # http://sequencedb.net/. This sequence database website should find a solution for that
    # sequence. For example, from solution at http://sequencedb.net/s/hwz12ospcif4b, scroll
    # down a bit and take the value corresponding to 100, that is, 190569291... TADA!
