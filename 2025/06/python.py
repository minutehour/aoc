#!/usr/bin/env python3

from math import prod
from fileinput import input

lines = [line.strip() for line in input()]
nums = [map(int, row.split()) for row in lines[:-1]]
fs = [prod if s == "*" else sum for s in lines[-1].split()]


silver = sum(f(ns) for f, ns in zip(fs, zip(*nums)))
gold = 0

print("silver:", silver)
print("gold:", gold)
