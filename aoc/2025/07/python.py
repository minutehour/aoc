#!/usr/bin/env python3

from fileinput import input
from functools import cache

lines = [line.strip() for line in input()]
start = lines[0].index("S")

silver = 0
bs = {start}
for line in lines:
    for b in bs.copy():
        if line[b] == "^":
            silver += 1
            bs -= {b}
            bs |= {b - 1, b + 1}


@cache
def tls(r: int, c: int) -> int:
    if r == len(lines):
        return 1
    if lines[r][c] == "^":
        return tls(r, c - 1) + tls(r, c + 1)
    return tls(r + 1, c)


gold = tls(0, start)

print("silver:", silver)
print("gold:", gold)
