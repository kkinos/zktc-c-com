int test1()
{
	int x = 3;
	return *&x;
}

int test2()
{
	int x = 3;
	int *y = &x;
	int **z = &y;
	return **z;
}

int test3()
{
	int x = 3;
	int y = 5;
	return *(&x - 1);
}

int test4()
{
	int x = 3;
	int y = 5;
	return *(&y + 1);
}

int test5()
{
	int x = 3;
	int *y = &x;
	*y = 5;
	return x;
}

int test6()
{
	int x[2];
	int *y = &x;
	*y = 3;
	return *x;
}

int test7()
{
	int x[3];
	*x = 3;
	*(x + 1) = 4;
	*(x + 2) = 5;
	return *x;
}

int test8()
{
	int x[3];
	*x = 3;
	*(x + 1) = 4;
	*(x + 2) = 5;
	return *(x + 1);
}

int test9()
{
	int x[3];
	*x = 3;
	*(x + 1) = 4;
	*(x + 2) = 5;
	return *(x + 2);
}

int main()
{
	assert(3, test1(), 1);
	assert(3, test2(), 2);
	assert(5, test3(), 3);
	assert(3, test4(), 4);
	assert(5, test5(), 5);
	assert(3, test6(), 6);
	assert(3, test7(), 7);
	assert(4, test8(), 8);
	assert(5, test9(), 9);
	return 0;
}
