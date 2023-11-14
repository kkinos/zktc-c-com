int test1()
{
	int i = 0;
	for (; i < 10; i = i + 1)
	{
		if (i == 3)
			break;
	}
	return i;
}

int test2()
{
	int i = 0;
	while (1)
	{
		if (i == 3)
			break;
		i = i + 1;
	}
	return i;
}
int test3()
{
	int i = 0;
	for (; i < 10; i = i + 1)
	{
		for (;;)
		{
			break;
		}
		if (i == 3)
			break;
	}
	return i;
}

int test4()
{
	int i = 0;
	while (1)
	{
		while (1)
		{
			break;
		}
		if (i == 3)
			break;
		i = i + 1;
	}

	return i;
}

int test5()
{
	int i = 0;
	int j = 0;
	for (; i < 10; i = i + 1)
	{
		if (i > 5)
			continue;
		j = j + 1;
	}
	return i;
}

int test6()
{
	int i = 0;
	int j = 0;
	for (; i < 10; i = i + 1)
	{
		if (i > 5)
			continue;
		j = j + 1;
	}
	return j;
}

int main()
{
	assert(3, test1(), 1);
	assert(3, test2(), 2);
	assert(3, test3(), 3);
	assert(3, test4(), 4);
	assert(10, test5(), 5);
	assert(6, test6(), 6);

	return 0;
}
