#!/usr/bin/env python3

from fileinput import input
from itertools import accumulate


def fn(p: tuple[int, int], rot: str) -> tuple[int, int]:
    x = p[0] + int(rot[1:]) * (1 if rot[0] == "R" else -1)
    return x % 100, abs(x) // 100 + (p[0] and x <= 0)


p1, p2 = zip(*list(accumulate(input(), fn, initial=(50, 0))))

print(f"silver: {sum(not p for p in p1)}")
print(f"gold: {sum(p2)}")
