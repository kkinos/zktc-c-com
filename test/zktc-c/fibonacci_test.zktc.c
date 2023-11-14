int fibonacci(int a)
{
	if (a == 0)
		return 0;
	if (a == 1)
		return 1;

	return fibonacci(a - 1) + fibonacci(a - 2);
}

int main()
{
	assert(1, fibonacci(1), 1);
	assert(5, fibonacci(5), 2);
	assert(55, fibonacci(10), 3);
	return 0;
}
