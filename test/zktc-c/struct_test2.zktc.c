int test1()
{
	struct
	{
		struct
		{
			char b;
		} a;
	} x;

	x.a.b = 6;

	return x.a.b;
}

int test2()
{
	struct
	{
		int a;
	} x;

	return sizeof(x);
}

int test3()
{
	struct
	{
		int a;
		int b;
	} x;

	return sizeof(x);
}

int test4()
{
	struct
	{
		int a[3];
	} x;

	return sizeof(x);
}

int test5()
{
	struct
	{
		int a;
	} x[4];

	return sizeof(x);
}

int test6()
{
	struct
	{
		char a;
		char b;
	} x;

	return sizeof(x);
}

int test7()
{
	struct
	{
		char a;
		int b;
	} x;

	return sizeof(x);
}

int test8()
{
	struct
	{

	} x;

	return sizeof(x);
}

int main()
{
	assert(6, test1(), 1);
	assert(2, test2(), 2);
	assert(4, test3(), 3);
	assert(6, test4(), 4);
	assert(8, test5(), 5);
	assert(2, test6(), 6);
	assert(3, test7(), 7);
	assert(0, test8(), 8);

	return 0;
}
