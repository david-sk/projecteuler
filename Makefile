.PHONY: list run-rust run-python run-cpp optimized-run-rust optimized-run-cpp

.DEFAULT_GOAL := list

SHELL := /bin/bash


list:
	# Listing of available make targets
	@(grep -oe '^[[:lower:][:digit:]_\-]\{1,\}:' Makefile | tr -d ':' | uniq)

run-rust:
	cargo run $(PROBLEM) $(VERSION)

run-python:
	python3 -m src $(PROBLEM) $(VERSION)

run-cpp:
	g++ src/main.cpp -o cpp_projecteuler -std=c++2a && ./cpp_projecteuler $(PROBLEM) $(VERSION)

optimized-run-rust:
	cargo build --release && ./target/release/projecteuler $(PROBLEM) $(VERSION)

optimized-run-cpp:
	g++ src/main.cpp -o cpp_projecteuler -O2 -std=c++2a && ./cpp_projecteuler $(PROBLEM) $(VERSION)

r-%: LANGUAGE ?= $*
r-%: V ?= v1
r-%:
	@if [ "$(LANGUAGE)" = "rs" ]; then make optimized-run-rust PROBLEM=$(P) VERSION=$(V); \
	 elif [ "$(LANGUAGE)" = "py" ]; then make run-python PROBLEM=$(P) VERSION=$(V); \
	 elif [ "$(LANGUAGE)" = "cpp" ]; then make optimized-run-cpp PROBLEM=$(P) VERSION=$(V); \
	 else echo "Invalid language extension specified: $(LANGUAGE)"; fi
