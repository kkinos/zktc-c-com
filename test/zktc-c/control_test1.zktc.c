int test1()
{
	int x;
	if (0)
		x = 2;
	else
		x = 3;

	return x;
}

int test2()
{
	int x;
	if (1 - 1)
		x = 2;
	else
		x = 3;

	return x;
}
int test3()
{
	int x;
	if (1)
		x = 2;
	else
		x = 3;

	return x;
}

int test4()
{
	int x;
	if (2 - 1)
		x = 2;
	else
		x = 3;

	return x;
}

int test5()
{
	int i;
	int j = 0;
	for (i = 0; i <= 10; i = i + 1)
		j = i + j;

	return j;
}

int test6()
{
	int i = 0;
	while (i < 10)
		i = i + 1;

	return i;
}

int main()
{
	assert(3, test1(), 1);
	assert(3, test2(), 2);
	assert(2, test3(), 3);
	assert(2, test4(), 4);

	assert(55, test5(), 5);

	assert(10, test6(), 6);

	return 0;
}
