#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/arith_test1.zktc.c  -o asm/arith_test1.asm
zktc-asm asm/arith_test1.asm -o mem/arith_test1.mem -b 0xb000


echo "=== arith test1 ==="


check mem/arith_test1.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/arith_test2.zktc.c  -o asm/arith_test2.asm
zktc-asm asm/arith_test2.asm -o mem/arith_test2.mem -b 0xb000


echo "=== arith test2 ==="

check mem/arith_test2.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/arith_test3.zktc.c  -o asm/arith_test3.asm
zktc-asm asm/arith_test3.asm -o mem/arith_test3.mem -b 0xb000


echo "=== arith test3 ==="

check mem/arith_test3.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/arith_test4.zktc.c  -o asm/arith_test4.asm
zktc-asm asm/arith_test4.asm -o mem/arith_test4.mem -b 0xb000


echo "=== arith test4 ==="

check mem/arith_test4.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/arith_test5.zktc.c  -o asm/arith_test5.asm
zktc-asm asm/arith_test5.asm -o mem/arith_test5.mem -b 0xb000


echo "=== arith test5 ==="

check mem/arith_test5.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/arith_test6.zktc.c  -o asm/arith_test6.asm
zktc-asm asm/arith_test6.asm -o mem/arith_test6.mem -b 0xb000


echo "=== arith test6 ==="

check mem/arith_test6.mem