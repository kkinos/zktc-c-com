int main()
{
	assert(1, 1 > 0, 1);
	assert(0, 1 > 1, 2);
	assert(0, 1 > 2, 3);
	assert(1, 1 >= 0, 4);
	assert(1, 1 >= 1, 5);
	assert(0, 1 >= 2, 6);
	assert(0, 1 & 0, 7);
	assert(3, 7 & 3, 8);
	assert(10, -1 & 10, 9);

	return 0;
}