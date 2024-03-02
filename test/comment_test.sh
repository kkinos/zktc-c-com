#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/comment_test.zktc.c -o asm/comment_test.asm
zktc-asm asm/comment_test.asm -o mem/comment_test.mem -b 0xb000


echo "=== comment test ==="

check mem/comment_test.mem