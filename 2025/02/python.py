#!/usr/bin/env python3

xs = [
    str(x)
    for r in input().split(",")
    for [a, b] in [r.split("-")]
    for x in range(int(a), int(b) + 1)
]


def invalid(x: str, n: int):
    p, q = divmod(len(x), n)
    return (
        p != 0
        and q == 0
        and len(
            {
                x[a:b]
                for a, b in zip(
                    range(0, len(x) + 1, p),
                    range(p, len(x) + 1, p),
                )
            }
        )
        == 1
    )


silver = sum(int(x) for x in xs if invalid(x, 2))
gold = sum(int(x) for x in xs if any(invalid(x, n) for n in range(2, len(x) + 1)))

print("silver:", silver)
print("gold:", gold)
