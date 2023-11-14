int test1()
{
	int a;
	a = 3;
	return a;
}

int test2()
{
	int a = 3;
	return a;
}

int test3()
{
	int a;
	int b;
	a = b = 3;
	return a + b;
}

int test4()
{
	int x;
	return sizeof(x);
}

int test5()
{
	int *x;
	return sizeof(x);
}

int test6()
{
	int x[4];
	return sizeof(x);
}

int test7()
{
	int x[3][4];
	return sizeof(x);
}

int test8()
{
	int x[3][4];
	return sizeof(*x);
}

int test9()
{
	int x[3][4];
	return sizeof(**x);
}

int main()
{
	assert(3, test1(), 1);
	assert(3, test2(), 2);
	assert(6, test3(), 3);
	assert(2, test4(), 4);
	assert(2, test5(), 5);
	assert(8, test6(), 6);
	assert(24, test7(), 7);
	assert(8, test8(), 8);
	assert(2, test9(), 9);

	return 0;
}