int test1()
{
	int x[2][3];
	int *y = x;
	y[0] = 0;
	return x[0][0];
}

int test2()
{
	int x[2][3];
	int *y = x;
	y[1] = 1;
	return x[0][1];
}

int test3()
{
	int x[2][3];
	int *y = x;
	y[2] = 2;
	return x[0][2];
}

int test4()
{
	int x[2][3];
	int *y = x;
	y[3] = 3;
	return x[1][0];
}

int test5()
{
	int x[2][3];
	int *y = x;
	y[4] = 4;
	return x[1][1];
}

int test6()
{
	int x[2][3];
	int *y = x;
	y[5] = 5;
	return x[1][2];
}

int main()
{
	assert(0, test1(), 1);
	assert(1, test2(), 2);
	assert(2, test3(), 3);
	assert(3, test4(), 4);
	assert(4, test5(), 5);
	assert(5, test6(), 6);

	return 0;
}
