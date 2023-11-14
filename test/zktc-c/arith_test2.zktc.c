int main()
{
	assert(0, 0 == 1, 1);
	assert(1, 42 == 42, 2);

	assert(1, 0 < 1, 3);
	assert(0, 1 < 1, 4);
	assert(0, 2 < 1, 5);
	assert(1, 0 <= 1, 6);
	assert(1, 1 <= 1, 7);
	assert(0, 2 <= 1, 8);

	return 0;
}