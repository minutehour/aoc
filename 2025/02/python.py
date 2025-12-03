#!/usr/bin/env python3


from itertools import accumulate
from bisect import bisect_left, bisect_right
from functools import cache

rs = [(int(a), int(b)) for r in input().split(",") for [a, b] in [r.split("-")]]
N = max(len(str(x)) for r in rs for x in r)


@cache
def memo(p2: bool):
    ids = sorted(
        {
            int(str(n) * reps)
            for n in range(10 ** (N // 2))
            for reps in range(2, N // len(str(n)) + 1 if p2 else 3)
        }
    )
    sums = list(accumulate(ids))
    return ids[1:], sums


def prefix_sum(a: int, b: int, ids: list[int], sums: list[int]):
    return sums[bisect_right(ids, b)] - sums[bisect_left(ids, a)]


silver = sum(prefix_sum(*r, *memo(False)) for r in rs)
gold = sum(prefix_sum(*r, *memo(True)) for r in rs)

print("silver:", silver)
print("gold:", gold)
