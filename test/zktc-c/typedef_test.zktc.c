typedef int MyInt;
typedef int MyInt2[4];

int test1()
{
	typedef int t;
	t x = 1;

	return x;
}

int test2()
{
	typedef struct
	{
		int a;
	} t;
	t x;
	x.a = 1;

	return x.a;
}
int test3()
{
	typedef int t;
	t t = 1;

	return t;
}

int test4()
{
	MyInt x = 3;

	return x;
}

int test5()
{
	MyInt2 x;

	return sizeof(x);
}

int main()
{
	assert(1, test1(), 1);
	assert(1, test2(), 2);
	assert(1, test3(), 3);
	assert(3, test4(), 4);
	assert(8, test5(), 5);

	return 0;
}
