int main()
{
	assert(0, ""[0], 1);
	assert(1, sizeof(""), 2);

	assert(97, "abc"[0], 3);
	assert(98, "abc"[1], 4);
	assert(99, "abc"[2], 5);
	assert(0, "abc"[3], 6);
	assert(4, sizeof("abc"), 7);

	assert(9, "\t"[0], 8);
	assert(10, "\n"[0], 9);

	return 0;
}