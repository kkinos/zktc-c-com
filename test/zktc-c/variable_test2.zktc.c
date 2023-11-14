int g1;
int g2[4];

int test1()
{
	return g1;
}

int test2()
{
	g1 = 3;
	return g1;
}

int test3()
{
	g2[0] = 0;
	g2[1] = 1;
	g2[2] = 2;
	g2[3] = 3;
	return g2[0];
}

int test4()
{
	g2[0] = 0;
	g2[1] = 1;
	g2[2] = 2;
	g2[3] = 3;
	return g2[1];
}

int test5()
{
	g2[0] = 0;
	g2[1] = 1;
	g2[2] = 2;
	g2[3] = 3;
	return g2[2];
}

int test6()
{
	g2[0] = 0;
	g2[1] = 1;
	g2[2] = 2;
	g2[3] = 3;
	return g2[3];
}

int test7()
{
	return sizeof(g1);
}

int test8()
{
	return sizeof(g2);
}

int main()
{
	assert(0, test1(), 1);
	assert(3, test2(), 2);
	assert(0, test3(), 3);
	assert(1, test4(), 4);
	assert(2, test5(), 5);
	assert(3, test6(), 6);
	assert(2, test7(), 7);
	assert(8, test8(), 8);

	return 0;
}