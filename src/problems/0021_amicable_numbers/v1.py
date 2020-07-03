#
# Amicable numbers, v1
# https://projecteuler.net/problem=21
#
# Let d(n) be defined as the sum of proper divisors of n
# (numbers less than n which divide evenly into n).
# If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are
# called amicable numbers.
# For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
# therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
# Evaluate the sum of all the amicable numbers under 10000.
#


def get_proper_divisors_sum(n):
    return 1 + sum(
        i + (n // i if i * i != n else 0) for i in range(2, int(n ** 0.5) + 1) if n % i == 0
    )


def run():
    limit = 10000

    amicable_numbers_sum = 0
    added_amicable_numbers = set()

    for i in range(1, limit):
        if i in added_amicable_numbers:
            added_amicable_numbers.remove(i)
            continue

        proper_divisors_sum_i = get_proper_divisors_sum(i)

        for j in range(i + 1, limit):
            if proper_divisors_sum_i == j and get_proper_divisors_sum(j) == i:
                amicable_numbers_sum += i + j
                added_amicable_numbers.add(j)
                break

    print('Amicable numbers sum:', amicable_numbers_sum)
