int main()
{
	x = 1;
	assert(1, x, 1);
	assert(2, sizeof(x), 2);

	y[0] = 2;
	assert(2, y[0], 3);
	assert(20, sizeof(y), 4);

	struct t z;
	z.a = 1;
	z.b = 3;
	assert(4, sizeof(z), 5);
	assert(4, z.a + z.b, 6);

	t1 w;
	w.a = 1;
	w.b = 3;
	assert(4, sizeof(w), 7);
	assert(4, w.a + w.b, 8);

	assert(3, M1, 9);

	return 0;
}