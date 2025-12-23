#!/usr/bin/env python3

from fileinput import input

pts = [[*map(int, line.split(","))] for line in input()]

silver = max(
    (abs(x1 - x2) + 1) * (abs(y1 - y2) + 1) for (x1, y1) in pts for (x2, y2) in pts
)
gold = 0

print("silver:", silver)
print("gold:", gold)
