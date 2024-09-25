#!/bin/bash
TARGET_DIR="target/bindings/kotlin"
PROJECT_DIR="bitcoin-ffi-jvm"
PACKAGE_DIR="org/bitcoindevkit/bitcoinffi"
UNIFFI_BINDGEN_BIN="cargo run --manifest-path uniffi-bindgen/Cargo.toml"

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
	rustup target add x86_64-unknown-linux-gnu || exit 1
	cargo build --release --target x86_64-unknown-linux-gnu || exit 1
	DYNAMIC_LIB_PATH="target/x86_64-unknown-linux-gnu/release/libbitcoin_ffi.so"
	RES_DIR="$PROJECT_DIR/lib/src/main/resources/linux-x86-64/"
	mkdir -p $RES_DIR || exit 1
	cp $DYNAMIC_LIB_PATH $RES_DIR || exit 1
else
	rustup target add x86_64-apple-darwin || exit 1
	cargo build --release --target x86_64-apple-darwin || exit 1
	DYNAMIC_LIB_PATH="target/x86_64-apple-darwin/release/libbitcoin_ffi.dylib"
	RES_DIR="$PROJECT_DIR/lib/src/main/resources/darwin-x86-64/"
	mkdir -p $RES_DIR || exit 1
	cp $DYNAMIC_LIB_PATH $RES_DIR || exit 1

	rustup target add aarch64-apple-darwin || exit 1
	cargo build --release --target aarch64-apple-darwin || exit 1
	DYNAMIC_LIB_PATH="target/aarch64-apple-darwin/release/libbitcoin_ffi.dylib"
	RES_DIR="$$PROJECT_DIR/lib/src/main/resources/darwin-aarch64/"
	mkdir -p $RES_DIR || exit 1
	cp $DYNAMIC_LIB_PATH $RES_DIR || exit 1
fi

mkdir -p "$PROJECT_DIR"/lib/src/main/kotlin/"$PACKAGE_DIR" || exit 1
$UNIFFI_BINDGEN_BIN generate src/bitcoin.udl --language kotlin -o "$TARGET_DIR" || exit 1

cp "$TARGET_DIR"/"$PACKAGE_DIR"/bitcoin.kt "$PROJECT_DIR"/lib/src/main/kotlin/"$PACKAGE_DIR"/bitcoinffi.kt || exit 1
