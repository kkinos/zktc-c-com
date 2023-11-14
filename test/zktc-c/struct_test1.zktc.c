int test1()
{
	struct
	{
		int a;
		int b;
	} x;

	x.a = 1;
	x.b = 2;

	return x.a;
}

int test2()
{
	struct
	{
		int a;
		int b;
	} x;

	x.a = 1;
	x.b = 2;

	return x.b;
}

int test3()
{
	struct
	{
		int a;
		int b;
		char c;
	} x;

	x.a = 1;
	x.b = 2;
	x.c = 3;

	return x.c;
}

int test4()
{
	struct
	{
		char a;
		char b;
	} x[3];

	char *p = x;
	p[0] = 0;

	return x[0].a;
}

int test5()
{
	struct
	{
		char a;
		char b;
	} x[3];

	char *p = x;
	p[1] = 1;

	return x[0].b;
}

int test6()
{
	struct
	{
		char a;
		char b;
	} x[3];

	char *p = x;
	p[2] = 2;

	return x[1].a;
}

int test7()
{
	struct
	{
		char a;
		char b;
	} x[3];

	char *p = x;
	p[3] = 3;

	return x[1].b;
}

int test8()
{
	struct
	{
		char a[3];
		char b[5];
	} x;

	char *p = &x;
	x.a[0] = 6;

	return p[0];
}

int test9()
{
	struct
	{
		char a[3];
		char b[5];
	} x;

	char *p = &x;
	x.b[0] = 7;

	return p[3];
}

int main()
{
	assert(1, test1(), 1);
	assert(2, test2(), 2);
	assert(3, test3(), 3);
	assert(0, test4(), 4);
	assert(1, test5(), 5);
	assert(2, test6(), 6);
	assert(3, test7(), 7);
	assert(6, test8(), 8);
	assert(7, test9(), 9);

	return 0;
}
