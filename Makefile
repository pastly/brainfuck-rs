.PHONY: build test test-slow test-all

build:
	cargo build

test: build
	./test.py

test-slow: build
	SLOW=1 ./test.py 

test-all: test-slow
