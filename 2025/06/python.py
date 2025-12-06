#!/usr/bin/env python3

from math import prod
from fileinput import input
from itertools import takewhile, repeat

*nums, ops = [line.strip("\n") for line in input()]
ops = [prod if op == "*" else sum for op in ops.split()]

horizontal = [map(int, row.split()) for row in nums]
silver = sum(op(chunk) for op, chunk in zip(ops, zip(*horizontal)))

vertical = ("".join(ds).strip() for ds in [*zip(*nums)])
chunks = repeat(lambda: [*map(int, takewhile(bool, vertical))])
gold = sum(op(chunk()) for op, chunk in zip(ops, chunks))

print("silver:", silver)
print("gold:", gold)
