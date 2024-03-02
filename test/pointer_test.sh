#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/pointer_test1.zktc.c -o asm/pointer_test1.asm
zktc-asm asm/pointer_test1.asm -o mem/pointer_test1.mem -b 0xb000


echo "=== pointer test1 ==="

check mem/pointer_test1.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/pointer_test2.zktc.c -o asm/pointer_test2.asm
zktc-asm asm/pointer_test2.asm -o mem/pointer_test2.mem -b 0xb000


echo "=== pointer test2 ==="

check mem/pointer_test2.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/pointer_test3.zktc.c -o asm/pointer_test3.asm
zktc-asm asm/pointer_test3.asm -o mem/pointer_test3.mem -b 0xb000


echo "=== pointer test3 ==="

check mem/pointer_test3.mem