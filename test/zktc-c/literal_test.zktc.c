int main()
{
	assert(97, "a"[0], 1);
	assert(10, "\n"[0], 2);

	assert(0, 0x0, 3);
	assert(10, 0xA, 4);
	assert(48879, 0xbeef, 5);

	return 0;
}