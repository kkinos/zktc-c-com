#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/variable_test1.zktc.c -o asm/variable_test1.asm
zktc-asm asm/variable_test1.asm -o mem/variable_test1.mem -b 0x8000


echo "=== variable test1 ==="

check mem/variable_test1.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/variable_test2.zktc.c -o asm/variable_test2.asm
zktc-asm asm/variable_test2.asm -o mem/variable_test2.mem -b 0x8000


echo "=== variable test2 ==="

check mem/variable_test2.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/variable_test3.zktc.c -o asm/variable_test3.asm
zktc-asm asm/variable_test3.asm -o mem/variable_test3.mem -b 0x8000


echo "=== variable test3 ==="

check mem/variable_test3.mem