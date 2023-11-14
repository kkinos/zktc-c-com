int test1()
{
	struct t
	{
		int a;
		int b;
	} x;

	return sizeof(x);
}

int test2()
{
	struct t
	{
		int a;
	};

	struct t y;

	return sizeof(y);
}

int test3()
{
	struct t
	{
		int x;
	};
	int t = 1;
	struct t y;
	y.x = 3;

	return t + y.x;
}

int test4()
{
	struct t
	{
		int a;
	} x;

	struct t *y = &x;
	x.a = 3;

	return y->a;
}

int test5()
{
	struct t
	{
		int a;
	} x;

	struct t *y = &x;
	y->a = 3;

	return x.a;
}

int main()
{
	assert(4, test1(), 1);
	assert(2, test2(), 2);
	assert(4, test3(), 3);
	assert(3, test4(), 4);
	assert(3, test5(), 5);

	return 0;
}
