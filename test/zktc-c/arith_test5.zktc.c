int main()
{
	assert(1, 1 << 0, 1);
	assert(8, 1 << 3, 2);
	assert(10, 5 << 1, 3);
	assert(2, 5 >> 1, 4);
	assert(-1, -1 >>> 1, 5);

	return 0;
}