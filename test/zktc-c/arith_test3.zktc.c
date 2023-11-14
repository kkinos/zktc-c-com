int main()
{
	assert(1, 1 > 0, 1);
	assert(0, 1 > 1, 2);
	assert(0, 1 > 2, 3);
	assert(1, 1 >= 0, 4);
	assert(1, 1 >= 1, 5);
	assert(0, 1 >= 2, 6);

	return 0;
}