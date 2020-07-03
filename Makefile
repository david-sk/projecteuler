.PHONY: list run optimized-run

.DEFAULT_GOAL := list

SHELL := /bin/bash


list:
	# Listing of available make targets
	@(grep -oe '^[[:lower:][:digit:]_\-]\{1,\}:' Makefile | tr -d ':' | uniq)

run:
ifeq ($(suffix $(FILE)),.rs)
	cargo run $(PROBLEM) $(basename $(FILE))
else
ifeq ($(suffix $(FILE)),.py)
	python3 -m src $(PROBLEM) $(basename $(FILE))
else
ifeq ($(suffix $(FILE)),.cpp)
	g++ src/main.cpp -o cpp_projecteuler && ./cpp_projecteuler $(PROBLEM) $(basename $(FILE))
else
	@(echo "/!\ Unsupported problem file /!\")
endif
endif
endif

optimized-run:
ifeq ($(suffix $(FILE)),.rs)
	cargo build --release && ./target/release/projecteuler $(PROBLEM) $(basename $(FILE))
else
ifeq ($(suffix $(FILE)),.cpp)
	g++ src/main.cpp -o cpp_projecteuler -O2 && ./cpp_projecteuler $(PROBLEM) $(basename $(FILE))
else
	@(echo "/!\ Unsupported problem file for optimized run /!\")
endif
endif
