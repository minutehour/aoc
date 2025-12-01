#!/usr/bin/env python3

from fileinput import input
from itertools import accumulate

lines = [line.strip() for line in input()]

add = lambda p, m: (p + int(m[1:]) * ((m[0] == "R") * 2 - 1)) % 100

silver = sum(p == 0 for p in accumulate(lines, add, initial=50))
gold = 0

print("silver:", silver)
print("gold:", gold)
