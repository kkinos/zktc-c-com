int main()
{
	assert(-10, 2 * -5, 1);
	assert(-10, -2 * 5, 2);
	assert(10, -2 * -5, 3);
	assert(-3, 6 / -2, 4);
	assert(-3, -6 / 2, 5);
	assert(3, -6 / -2, 6);

	return 0;
}