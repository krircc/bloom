.PHONY: build clean re dev test build_static
.PHONY: disposable_emails lint fmt fmt_check lint

DIST_DIR = dist
NAME := bloom

all: build

build:
	mkdir -p $(DIST_DIR)
	cargo build -p api --release
	cp target/release/api $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

build_debug:
	mkdir -p $(DIST_DIR)
	cargo build -p api
	cp target/debug/api $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

build_static:
	mkdir -p $(DIST_DIR)
	cargo build -p api --release --target=x86_64-unknown-linux-musl
	cp target/x86_64-unknown-linux-musl/release/api $(DIST_DIR)/$(NAME)
	cp -r assets $(DIST_DIR)/

dev:
	cargo watch -x 'run -p api'

clean:
	rm -rf $(DIST_DIR) target/

re: clean build

test:
	cargo test

fmt:
	cargo fmt

fmt_check:
	cargo fmt --all -- --check

lint:
	cargo clippy -- -D warnings -A clippy::needless_return

audit:
	cargo audit


crates_login:
	cargo login ${CRATES_TOKEN}

crates_publish:
	cargo publish

disposable_emails:
	cd scripts && ./disposable_emails.sh
