#!/bin/bash

. ./check.sh

cargo run --  zktc-c/assert.zktc.c zktc-c/extern_test1.zktc.c zktc-c/extern_test2.zktc.c -o asm/extern_test.asm
zktc-asm asm/extern_test.asm -o mem/extern_test.mem -b 0xb000


echo "=== extern test ==="

check mem/extern_test.mem