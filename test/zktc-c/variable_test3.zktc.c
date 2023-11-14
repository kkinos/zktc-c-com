int test1()
{
	char a;
	a = 1;
	return a;
}

int test2()
{
	char a = 1;
	char b = 2;
	return a;
}

int test3()
{
	char a = 1;
	char b = 2;
	return b;
}

int test4()
{
	char x;
	return sizeof(x);
}

int test5()
{
	char x[10];
	return sizeof(x);
}

int main()
{
	assert(1, test1(), 1);
	assert(1, test2(), 2);
	assert(2, test3(), 3);
	assert(1, test4(), 4);
	assert(10, test5(), 5);

	return 0;
}