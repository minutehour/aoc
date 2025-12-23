#!/usr/bin/env python3

from fileinput import input
from functools import partial, reduce

lines = [line.strip() for line in input()]

bs = {lines[0].index("S")}
silver = 0


def split(t: int, c: int) -> set[int]:
    global silver
    if lines[t][c] == "^":
        silver += 1
        return {c - 1, c + 1}
    return {c}


for t, _ in enumerate(lines):
    bs = reduce(set.union, map(partial(split, t), bs))


gold = 0

print("silver:", silver)
print("gold:", gold)
