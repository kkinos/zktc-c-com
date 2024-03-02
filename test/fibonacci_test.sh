#!/bin/bash

. ./check.sh 

cargo run -- zktc-c/assert.zktc.c zktc-c/fibonacci_test.zktc.c -o asm/fibonacci_test.asm
zktc-asm asm/fibonacci_test.asm -o mem/fibonacci_test.mem -b 0xb000


echo "=== fibonacci test ==="

check mem/fibonacci_test.mem