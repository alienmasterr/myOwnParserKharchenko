BIN_NAME = arythemetic_expressions_parser_kharchenko_kma

.PHONY: run test fmt clippy check build clean all precommit help

run:
	cargo run

run-args:
	cargo run -- $(args)

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo check

build:
	cargo build

clean:
	cargo clean

precommit: fmt clippy test

all: clean fmt clippy build test

help:
	@echo "commands:"
	@echo "  make run                     - run the main program"
	@echo "  make run-args args=\"help\"  - run program with arguments"
	@echo "  make test                    - run tests"
	@echo "  make fmt                     - format code using cargo fmt"
	@echo "  make clippy                  - check if to refactor code using cargo clippy"
	@echo "  make precommit               - run fmt, clippy and tests before committing"
	@echo "  make all                     - full clean build + tests"
	@echo "  make clean                   - remove build artifacts"
