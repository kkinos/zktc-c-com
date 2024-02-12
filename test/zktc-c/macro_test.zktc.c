#define M1 3
#define M2 4
#define M3 0xbeef

int main()
{
	assert(3, M1, 1);
	assert(4, M2, 2);
	assert(48879, M3, 3);

	return 0;
}