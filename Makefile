BUILD_TYPE ?= debug

.PHONY: all

all: pkg/typst-pyrunner.wasm

pkg/typst-pyrunner.wasm: target/wasm32-wasi/${BUILD_TYPE}/typst-pyrunner.wasm
	wasi-stub target/wasm32-wasi/${BUILD_TYPE}/typst-pyrunner.wasm -o /tmp/typst-pyrunner.wasm --stub-function wasi_snapshot_preview1:random_get,wasi_snapshot_preview1:environ_get,wasi_snapshot_preview1:environ_sizes_get -r 0
	wasi-stub /tmp/typst-pyrunner.wasm -o pkg/typst-pyrunner.wasm --stub-module wasi_snapshot_preview1
