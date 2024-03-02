#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/macro_test.zktc.c -o asm/macro_test.asm
zktc-asm asm/macro_test.asm -o mem/macro_test.mem -b 0xb000


echo "=== macro test ==="

check mem/macro_test.mem