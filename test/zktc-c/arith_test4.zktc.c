int main()
{
	assert(1, 0 | 1, 1);
	assert(0b10011, 0b10000 | 0b00011, 2);
	assert(0, 0 ^ 0, 3);
	assert(0, 0b1111 ^ 0b1111, 4);
	assert(0b110100, 0b111000 ^ 0b001100, 5);

	assert(0, !1, 6);
	assert(0, !2, 7);
	assert(1, !0, 8);

	return 0;
}