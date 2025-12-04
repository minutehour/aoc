#!/usr/bin/env python3

from fileinput import input

g = {
    complex(x, y): ch
    for y, line in enumerate(input())
    for x, ch in enumerate(line.strip())
}


def accessible():
    return {
        p
        for p, x in g.items()
        if x == "@"
        and sum(
            1
            for dx in (-1, 0, 1)
            for dy in (-1j, 0j, 1j)
            if dx + dy != 0 and g.get(p + dx + dy, ".") == "@"
        )
        < 4
    }


silver = len(accessible())
gold = 0

while rem := accessible():
    gold += len(rem)
    for gone in rem:
        g.pop(gone)

print("silver:", silver)
print("gold:", gold)
