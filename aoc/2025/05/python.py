#!/usr/bin/env python3

from itertools import takewhile
from fileinput import input


inp = map(str.strip, input())
rs = [tuple(map(int, r.split("-"))) for r in takewhile(bool, inp)]
ids = list(map(int, inp))


def add(a: int, b: int, c: int, d: int) -> tuple[int, int] | None:
    if c < a:
        return add(c, d, a, b)
    if b < c:
        return None
    return (a, max(b, d))


done = False
while not done:
    ms = []
    for r in rs:
        for idx, m in enumerate(ms):
            if new := add(*r, *m):
                ms[idx] = new
                break

        else:
            ms.append(r)
    done = len(rs) == len(ms)
    rs = ms


silver = sum(1 for i in ids if any(lo <= i <= hi for lo, hi in rs))
gold = sum(b - a + 1 for a, b in rs)

print("silver:", silver)
print("gold:", gold)
