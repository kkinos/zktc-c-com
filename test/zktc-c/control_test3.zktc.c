int main()
{
	assert(1, 0 || 1, 1);
	assert(1, 2 || (2 - 2) || 5, 2);
	assert(0, 0 || 0, 3);
	assert(0, 0 || (2 - 2), 4);
	assert(0, 0 && 1, 5);
	assert(0, (2 - 2) && 5, 6);
	assert(1, 1 && 5, 7);

	return 0;
}
