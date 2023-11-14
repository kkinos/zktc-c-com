int assert(int expected, int actual, int num)
{
	if (expected == actual)
	{
		// If succeeds, returns 0
		return 0;
	}
	else
	{
		// If fails, returns num and trap
		__asm__("mov a0, a2");
		__asm__("trap");
	}
}