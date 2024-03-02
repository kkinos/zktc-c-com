#!/bin/bash

. ./check.sh

cargo run -- zktc-c/assert.zktc.c zktc-c/control_test1.zktc.c -o asm/control_test1.asm
zktc-asm asm/control_test1.asm -o mem/control_test1.mem -b 0xb000


echo "=== control test1 ==="

check mem/control_test1.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/control_test2.zktc.c -o asm/control_test2.asm
zktc-asm asm/control_test2.asm -o mem/control_test2.mem -b 0xb000


echo "=== control test2 ==="

check mem/control_test2.mem

cargo run -- zktc-c/assert.zktc.c zktc-c/control_test3.zktc.c -o asm/control_test3.asm
zktc-asm asm/control_test3.asm -o mem/control_test3.mem -b 0xb000


echo "=== control test3 ==="

check mem/control_test3.mem