PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin

.PHONY: build install test fmt-check

build:
	cargo build --locked --release

install: build
	install -Dm755 target/release/wrap $(BINDIR)/wrap

test:
	cargo test --locked

fmt-check:
	cargo fmt --check
