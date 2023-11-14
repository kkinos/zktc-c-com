#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/func_call_test.zktc.c -o asm/func_call_test.asm
zktc-asm asm/func_call_test.asm -o mem/func_call_test.mem -b 0x8000


echo "=== func call test ==="

check mem/func_call_test.mem