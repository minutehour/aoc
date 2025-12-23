#!/usr/bin/env python3

from fileinput import input
from math import dist, prod
from operator import itemgetter

MAX_CONNS = 1000  # test: 10, input: 1000

boxes = [[*map(int, line.split(","))] for line in input()]
N = len(boxes)

js = {n: {n} for n in range(N)}
conns = sorted(
    ((x, y) for x in range(N) for y in range(x)),
    key=lambda t: dist(*itemgetter(*t)(boxes)),
)

for n, (x, y) in enumerate(conns):
    if n == MAX_CONNS:
        lens = {id(j): len(j) for j in js.values()}.values()
        print("silver:", prod(sorted(lens, reverse=True)[:3]))

    j = js[x] | js[y]
    for b in j:
        js[b] = j

    if len(j) == N:
        print("gold:", boxes[x][0] * boxes[y][0])
        break
