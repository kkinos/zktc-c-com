#!/bin/bash

cd test

for test in `find -name '*_test.sh'`; do
	. $test
done

cd ..