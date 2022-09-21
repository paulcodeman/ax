.PHONY: build normal watch test clean generate

build:
	wasm-pack build --target web --release

normal:
	wasm-pack build --target web

example: build
	cd example
	npm i && npm run build

watch:
	cargo watch -s "make normal"

watch-tests:
	cargo watch --why --clear --exec 'tarpaulin --out Lcov --skip-clean' --ignore lcov.info

test:
	cargo test

generate:
	python3 generate.py generate

dependencies:
	cargo install cargo-tarpaulin cargo-watch
	python3 -m pip install pyperclip tqdm

clean:
	rm -rf pkg
