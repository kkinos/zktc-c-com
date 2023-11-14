#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/string_test.zktc.c -o asm/string_test.asm
zktc-asm asm/string_test.asm -o mem/string_test.mem -b 0x8000


echo "=== string test ==="

check mem/string_test.mem
