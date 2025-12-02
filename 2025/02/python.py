#!/usr/bin/env python3

from fileinput import input
from math import log10

rs = [[int(i) for i in r.split("-")] for r in "".join(input()).split(",")]


def invalid(i):
    d = int(log10(i)) + 1
    a, b = divmod(i, 10 ** (d // 2))
    return a == b


silver = sum(x for [a, b] in rs for x in range(a, b + 1) if invalid(x))
gold = 0

print("silver:", silver)
print("gold:", gold)
