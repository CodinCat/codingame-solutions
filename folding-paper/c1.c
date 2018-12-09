#include <stdlib.h>
#include <stdio.h>
#include <string.h>

struct paper {
	unsigned int R;
	unsigned int L;
	unsigned int U;
	unsigned int D;
};

int get_side(struct paper *p, char *side)
{
	if (strcmp(side, "R") == 0) return p->R;
	else if (strcmp(side, "L") == 0) return p->L;
	else if (strcmp(side, "U") == 0) return p->U;
	return p->D;
}

int main()
{
    char order[20];
    fgets(order, 20, stdin);
    char side[6];
    fgets(side, 6, stdin);

	struct paper p;
	p.R = 1;
	p.L = 1;
	p.U = 1;
	p.D = 1;

	for (int i = 0; i < strlen(order); i++) {
		switch (order[i]) {
		case 'R':
			p.L += p.R;
			p.R = 1;
			p.U *= 2;
			p.D *= 2;
			break;
		case 'L':
			p.R += p.L;
			p.L = 1;
			p.U *= 2;
			p.D *= 2;
			break;
		case 'U':
			p.D += p.U;
			p.U = 1;
			p.L *= 2;
			p.R *= 2;
			break;
		case 'D':
			p.U += p.D;
			p.D = 1;
			p.L *= 2;
			p.R *= 2;
			break;
		}
	}
    printf("%d\n", get_side(&p, side));
    return 0;
}
