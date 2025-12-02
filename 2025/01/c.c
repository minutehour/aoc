#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

int main()
{
	char dir;
	int64_t n;
	int64_t p = 50;
	int64_t p1 = 0, p2 = 0;
	while (scanf("%c%ld ", &dir, &n) == 2) {
		int64_t new = p + (dir == 'R' ? n : -n);
		p2 += labs(new) / 100 + (p != 0 && new <= 0);
		p = (p = new % 100) >= 0 ? p : p + 100;
		p1 += p == 0;
	}
	printf("silver: %ld\ngold: %ld", p1, p2);
	return 0;
}
