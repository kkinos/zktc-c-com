int main()
{
	assert(0, 0, 1);
	assert(42, 42, 2);
	assert(21, 5 + 20 - 4, 3);
	assert(47, 5 + 6 * 7, 4);
	assert(15, 5 * (9 - 6), 5);
	assert(4, (3 + 5) / 2, 6);
	assert(10, -10 + 20, 7);

	return 0;
}