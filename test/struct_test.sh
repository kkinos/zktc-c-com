#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/struct_test1.zktc.c -o asm/struct_test1.asm
zktc-asm asm/struct_test1.asm -o mem/struct_test1.mem -b 0xb000

echo "=== struct test1 ==="

check mem/struct_test1.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/struct_test2.zktc.c -o asm/struct_test2.asm
zktc-asm asm/struct_test2.asm -o mem/struct_test2.mem -b 0xb000

echo "=== struct test2 ==="

check mem/struct_test2.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/struct_test3.zktc.c -o asm/struct_test3.asm
zktc-asm asm/struct_test3.asm -o mem/struct_test3.mem -b 0xb000

echo "=== struct test3 ==="

check mem/struct_test3.mem