int main()
{
	assert(0, 10 % 5, 1);
	assert(3, 13 % 5, 2);
	assert(-3, -13 % 5, 3);
	assert(-1, -1 % 3, 4);
	assert(-1, -1 % 32767, 5);

	return 0;
}