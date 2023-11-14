.PHONY: test
test: $(TARGET)
	. ./all_test.sh

.PHONY: clean
clean: 
	rm -rf test/asm/*.asm
	rm -rf test/mem/*.mem