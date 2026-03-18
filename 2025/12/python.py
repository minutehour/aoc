#!/usr/bin/env python3

from fileinput import input
from itertools import takewhile


def check(s: str):
    return eval(s.replace(": ", ">=9*(").replace("x", "*").replace(" ", "+") + ")")


silver = sum(
    map(
        check,
        takewhile(
            bool,
            [line.strip() for line in input()][::-1],
        ),
    )
)

print("silver:", silver)
