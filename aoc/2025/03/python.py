#!/usr/bin/env python3

from fileinput import input

lines = [[int(b) for b in line.strip()] for line in input()]


def jolt(bs: list[int], n: int) -> int:
    if n == 1:
        return max(bs)
    n -= 1
    b = max(bs[:-n])
    i = bs.index(b)
    return b * (10**n) + jolt(bs[i + 1 :], n)


silver = sum(jolt(bs, 2) for bs in lines)
gold = sum(jolt(bs, 12) for bs in lines)

print("silver:", silver)
print("gold:", gold)
