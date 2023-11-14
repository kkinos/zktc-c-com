int ret3()
{
	return 3;
	return 5;
}

int add2(int x, int y)
{
	return x + y;
}

int sub2(int x, int y)
{
	return x - y;
}

int addx(int *x, int y)
{
	return *x + y;
}

int sub_char(char a, char b, char c)
{
	return a - b - c;
}

int sum(int m, int n)
{
	int acc = 0;
	for (int i = m; i <= n; i = i + 1)
		acc = acc + i;
	return acc;
}

int main()
{
	assert(3, ret3(), 1);
	assert(8, add2(3, 5), 2);
	assert(2, sub2(5, 3), 3);
	assert(7, add2(3, 4), 4);
	assert(1, sub2(4, 3), 5);
	assert(1, sub_char(7, 3, 3), 6);
	int *x;
	int y = 1;
	x = &y;
	assert(2, addx(x, 1), 7);
	assert(15, sum(1, 5), 8);

	return 0;
}