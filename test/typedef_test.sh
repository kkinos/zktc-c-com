#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/typedef_test.zktc.c -o asm/typedef_test.asm
zktc-asm asm/typedef_test.asm -o mem/typedef_test.mem -b 0xb000


echo "=== typedef test ==="

check mem/typedef_test.mem