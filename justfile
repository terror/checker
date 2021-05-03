all: build test clippy fmt

build:
  cargo build

test *args:
  cargo test --{{args}}

fmt:
  cargo +nightly fmt

run *args:
  cargo run -- {{args}}

publish:
  cargo publish

watch command='test':
	cargo watch --exec '{{command}}'

clippy:
  cargo clippy --all

help:
  cargo run -- --help
