#!/usr/bin/env python3

from fileinput import input

lines = [[int(b) for b in line.strip()] for line in input()]


def joltage(bs: list[int]):
    d2 = max(bs[:-1])
    return max(d2 * 10 + max(bs[idx + 1 :]) for idx, b in enumerate(bs[:-1]) if b == d2)


silver = sum(map(joltage, lines))
gold = 0

print("silver:", silver)
print("gold:", gold)
