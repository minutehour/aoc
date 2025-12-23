#!/usr/bin/env python3

from fileinput import input
from math import dist, prod
from operator import itemgetter

boxes = [[*map(int, line.split(","))] for line in input()]
N = len(boxes)

js = {n: {n} for n in range(N)}
conns = sorted(
    ((x, y) for x in range(N) for y in range(x)),
    key=lambda t: dist(*itemgetter(*t)(boxes)),
)

for x, y in conns[:1000]:
    j = js[x] | js[y]
    for b in j:
        js[b] = j

lens = {id(j): len(j) for j in js.values()}.values()

silver = prod(sorted(lens, reverse=True)[:3])
gold = 0

print("silver:", silver)
print("gold:", gold)
