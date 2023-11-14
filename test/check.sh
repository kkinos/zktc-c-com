#!/bin/bash

check () {

	res=$(expect -c "
		set timeout 3
		spawn zktc-emu $1
		expect \"zktc-emu >>\"
		send \"r\n\"
		expect \"zktc-emu >>\"
		send \"regs\n\"
		expect \"zktc-emu >>\"
	"  | sed -n -r 's/.*x3 : 0x000([a-z0-9]).*/\1/p')

	if [ "$res" = "0" ]; then
		echo "\e[32mPASSED"
	else
		echo "\e[31mTEST${res} FAILED"
		exit 1
	fi
}