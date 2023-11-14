#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/literal_test.zktc.c -o asm/literal_test.asm
zktc-asm asm/literal_test.asm -o mem/literal_test.mem -b 0x8000


echo "=== literal test ==="

check mem/literal_test.mem