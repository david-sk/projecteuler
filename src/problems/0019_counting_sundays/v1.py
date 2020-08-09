#
# Counting Sundays, v1
# https://projecteuler.net/problem=19
#
# You are given the following information, but you may prefer to do some research for yourself.
#     - 1 Jan 1900 was a Monday.
#     - Thirty days has September,
#       April, June and November.
#       All the rest have thirty-one,
#       Saving February alone,
#       Which has twenty-eight, rain or shine.
#     - And on leap years, twenty-nine.
# A leap year occurs on any year evenly divisible by 4, but not on a century unless it is
# divisible by 400.
# How many Sundays fell on the first of the month during the twentieth century
# (1 Jan 1901 to 31 Dec 2000)?
#

from datetime import date, timedelta


def run():
    # Why doing math when we can cheat with Python's datetime library :-P

    current_date = date(1901, 1, 1)
    last_date = date(2000, 12, 31)
    num_sundays = 0

    while current_date <= last_date:
        if current_date.day == 1:
            if current_date.weekday() == 6:
                num_sundays += 1
            current_date += timedelta(days=28)
        else:
            current_date += timedelta(days=1)

    print('Number of sundays:', num_sundays)
